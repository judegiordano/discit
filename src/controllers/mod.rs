mod dev;
mod discs;

pub fn routes() -> axum::Router {
    axum::Router::new()
        .nest("/dev", dev::router())
        .nest("/disc", discs::router())
}
