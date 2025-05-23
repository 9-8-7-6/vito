//! Scheduler Updater Modules
//!
//! These modules define periodic background jobs that fetch and update external data
//! into the local database. Each updater is responsible for managing one type of data source.
//!
//! - `country_info_updater` — Updates country metadata (name, region, timezones, flag)
//! - `stock_info_updater` — Retrieves daily trading data (e.g., open/close price, volume)
//! - `stock_meta_updater` — Maintains stock metadata such as ticker symbols and company names
pub mod country_info_updater;
pub mod stock_info_updater;
pub mod stock_meta_updater;

pub use country_info_updater::update_country_info_every_month;
pub use stock_info_updater::update_stock_info_every_day;
pub use stock_meta_updater::update_stock_metadata_every_month;
