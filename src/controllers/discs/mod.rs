use axum::routing::get;

mod controller;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(controller::list_all_discs))
        .route("/:id", get(controller::read_by_id))
}
