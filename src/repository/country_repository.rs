use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

use crate::models::{Country, CountryList};

const QUERY_ALL_COUNTRIES_INFO: &str = "SELECT * FROM countries";
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

pub async fn fetch_all_countries(pool: &PgPool) -> Result<CountryList, sqlx::Error> {
    let countries = sqlx::query_as::<_, Country>(QUERY_ALL_COUNTRIES_INFO)
        .fetch_all(pool)
        .await?;

    Ok(CountryList(countries))
}

pub async fn upsert_country(pool: &PgPool, datas: Vec<Country>) -> Result<(), sqlx::Error> {
    for data in datas {
        sqlx::query(QUERY_INSERT_OR_UPDATE_COUNTRY)
            .bind(Uuid::new_v4()) // id
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
