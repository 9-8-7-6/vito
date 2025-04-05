use axum::{
    routing::{get, post, put},
    Router,
};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::stock_handler::*;

pub fn stock_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/stock-holdings", get(get_all_stock_holdings_handler))
        .route(
            "/stock-holdings/{account_id}",
            get(get_stock_holdings_by_account_handler),
        )
        .route("/stock-holding", post(create_stock_holding_handler))
        .route(
            "/stock-holding/{id}",
            put(update_stock_holding_handler).delete(delete_stock_holding_handler),
        )
        .route(
            "/stock-metadata",
            get(get_all_stock_metadata_handler).post(create_stock_metadata_handler),
        )
        .route(
            "/stock-metadata/{id}",
            get(get_stock_metadata_by_id_handler)
                .put(update_stock_metadata_handler)
                .delete(delete_stock_metadata_handler),
        )
        // .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state)
}
