//! Scheduler API module
//!
//! This module provides API clients used by scheduled tasks to fetch external data.
//! It is organized by data type and region-specific logic.
//!
//! Submodules:
//! - `country_info` — Handles fetching and parsing of country and region metadata
//! - `stock_info` — Retrieves real-time or daily stock trading data (e.g., TWSE)
//! - `stock_metadata` — Fetches static stock listing metadata (e.g., ticker and company name)
//!                      from different exchanges like TWSE and US via Finnhub
pub mod country_info;
pub mod stock_info;
pub mod stock_metadata;
