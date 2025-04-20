use crate::models::Currency;
use sqlx::{PgPool, Postgres, QueryBuilder};

/// Performs a bulk upsert (insert or update) for a list of currencies into the database.
///
/// For each currency, if a record with the same `code` already exists,
/// it updates the `name` and `rate` fields with the new values.
/// Otherwise, it inserts a new record.
///
/// # Arguments
/// * `pool` - A reference to the PostgreSQL connection pool
/// * `currencies` - A vector of `Currency` structs to be inserted or updated
///
/// # Returns
/// * `Ok(())` if all operations succeed
/// * `Err(sqlx::Error)` if any database error occurs
pub async fn upsert_currencies(
    pool: &PgPool,
    currencies: Vec<Currency>,
) -> Result<(), sqlx::Error> {
    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO currencies (id, code, name, rate) ");

    builder.push_values(currencies.iter(), |mut b, currency| {
        b.push_bind(currency.id)
            .push_bind(&currency.code)
            .push_bind(&currency.name)
            .push_bind(&currency.rate);
    });

    builder.push(
        " ON CONFLICT (code) DO UPDATE SET \
           name = EXCLUDED.name, \
           rate = EXCLUDED.rate",
    );

    let query = builder.build();
    query.execute(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Currency;
    use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
    use std::env;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        dotenvy::from_filename(".env.test").ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test");

        if !Postgres::database_exists(&database_url)
            .await
            .unwrap_or(false)
        {
            Postgres::create_database(&database_url)
                .await
                .expect("Failed to create test database");
        }

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect");
        sqlx::migrate!().run(&pool).await.expect("Migration failed");
        pool
    }

    #[tokio::test]
    async fn test_upsert_currency() {
        let pool = setup_test_db().await;

        let mut currencies = vec![
            Currency {
                id: Uuid::new_v4(),
                code: "USD".to_string(),
                name: "United States Dollar".to_string(),
                rate: Some("1.00".to_string()),
            },
            Currency {
                id: Uuid::new_v4(),
                code: "EUR".to_string(),
                name: "Euro".to_string(),
                rate: Some("0.90".to_string()),
            },
        ];

        // Insert new records
        upsert_currencies(&pool, currencies.clone())
            .await
            .expect("Insert failed");

        // Update existing record
        currencies[1].name = "Euro Updated".to_string();
        currencies[1].rate = Some("0.91".to_string());

        upsert_currencies(&pool, currencies.clone())
            .await
            .expect("Update failed");

        let rows: Vec<Currency> = sqlx::query_as("SELECT * FROM currencies WHERE code = 'EUR'")
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "Euro Updated");
        assert_eq!(rows[0].rate.as_deref(), Some("0.91"));
    }
}
