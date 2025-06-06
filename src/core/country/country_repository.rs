use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Country, CountryList};

// SQL query to fetch all countries
const QUERY_ALL_COUNTRIES_INFO: &str = "SELECT * FROM countries";

// SQL upsert query to insert or update a country by its unique code
const QUERY_INSERT_OR_UPDATE_COUNTRY: &str = r#"
    INSERT INTO countries (id, code, name, region, subregion, timezone, flag_url)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    ON CONFLICT (code) DO UPDATE SET
        name = EXCLUDED.name,
        region = EXCLUDED.region,
        subregion = EXCLUDED.subregion,
        timezone = EXCLUDED.timezone,
        flag_url = EXCLUDED.flag_url;
"#;

/// Fetch all country records from the `countries` table
pub async fn fetch_all_countries(pool: &PgPool) -> Result<CountryList, sqlx::Error> {
    let countries = sqlx::query_as::<_, Country>(QUERY_ALL_COUNTRIES_INFO)
        .fetch_all(pool)
        .await?;

    Ok(CountryList(countries))
}

/// Insert or update a list of countries in the database.
///
/// For each country:
/// - If the `code` does not exist, a new record is inserted.
/// - If the `code` already exists, the corresponding record is updated.
pub async fn upsert_country(pool: &PgPool, datas: Vec<Country>) -> Result<(), sqlx::Error> {
    for data in datas {
        sqlx::query(QUERY_INSERT_OR_UPDATE_COUNTRY)
            .bind(Uuid::new_v4()) // Assign a new UUID for the country ID
            .bind(data.code)
            .bind(data.name)
            .bind(data.region)
            .bind(data.subregion)
            .bind(data.timezone)
            .bind(data.flag_url)
            .execute(pool)
            .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Country;
    use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
    use std::env;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        dotenvy::from_filename(".env.test").ok();
        let test_database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test");

        if !Postgres::database_exists(&test_database_url)
            .await
            .unwrap_or(false)
        {
            Postgres::create_database(&test_database_url)
                .await
                .expect("Failed to create test database");
        }

        let pool = PgPool::connect(&test_database_url)
            .await
            .expect("Failed to connect to the test database");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn test_upsert_and_fetch_countries() {
        let pool = setup_test_db().await;

        // Clean up any old entries
        sqlx::query("DELETE FROM countries")
            .execute(&pool)
            .await
            .unwrap();

        let countries = vec![
            Country {
                id: Uuid::new_v4(),
                code: "TW".to_string(),
                name: "Taiwan".to_string(),
                region: Some("Asia".to_string()),
                subregion: Some("Eastern Asia".to_string()),
                timezone: Some(vec!["Asia/Taipei".to_string()]),
                flag_url: Some("https://flagcdn.com/tw.svg".to_string()),
            },
            Country {
                id: Uuid::new_v4(),
                code: "JP".to_string(),
                name: "Japan".to_string(),
                region: Some("Asia".to_string()),
                subregion: Some("Eastern Asia".to_string()),
                timezone: Some(vec!["Asia/Tokyo".to_string()]),
                flag_url: Some("https://flagcdn.com/jp.svg".to_string()),
            },
        ];

        // Upsert countries
        upsert_country(&pool, countries.clone()).await.unwrap();

        // Fetch countries and verify
        let result = fetch_all_countries(&pool).await.unwrap();
        assert_eq!(result.0.len(), 2);
        assert!(result.0.iter().any(|c| c.code == "TW"));
        assert!(result.0.iter().any(|c| c.code == "JP"));
    }
}
