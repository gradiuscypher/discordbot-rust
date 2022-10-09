use axum::routing::post;
use axum::{Router, Server};
use interactions::handle_interaction;
use log::info;
use log4rs;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let app = Router::new()
        .route("/interact", post(handle_interaction))
        .into_make_service();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("serving on {addr}");
    Server::bind(&addr).serve(app).await.unwrap();
}
