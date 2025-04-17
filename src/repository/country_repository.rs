use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
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
