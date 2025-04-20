//! Scheduler Module
//!
//! This module manages background tasks for periodically fetching and updating data
//! from external APIs (such as stock metadata, stock info, and country info).
//!
//! - `api` contains HTTP request logic for retrieving external data.
//! - `tasks` contains scheduled job implementations that run at specific intervals.
//! - `scheduler_launcher` provides the entry point to start all background schedulers.
//!
//! Use `start_all_schedulers()` to launch all recurring background jobs when the application starts.
pub mod api;
pub mod tasks;
