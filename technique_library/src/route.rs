use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_ma_handler, delete_ma_handler, edit_ma_handler,
        get_ma_handler, health_check_handler, ma_list_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/", get(|| async { "Martial Arts Site" }))
    .route("/api/healthchecker", get(health_check_handler))
    .route("/api/martial_arts/", post(create_ma_handler))
    .route("/api/martial_arts", get(ma_list_handler))
    .route(
        "/api/martial_arts/:id",
        get(get_ma_handler)
            .patch(edit_ma_handler)
            .delete(delete_ma_handler)
    )
    .with_state(app_state)
}