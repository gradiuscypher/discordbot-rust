use axum::routing::post;
use axum::{Router, Server};
use discordbot_rust::handle_interaction;
use std::env;

static TOKEN_VAR: &str = "DISCORD_BOT_TOKEN";
static APPID_VAR: &str = "DISCORD_APP_ID";

#[tokio::main]
async fn main() {
    let bot_token = env::var(TOKEN_VAR).expect(&format!("{TOKEN_VAR} unset"));
    let app_id = env::var(APPID_VAR)
        .expect(&format!("{APPID_VAR} unset"))
        .parse::<u64>()
        .unwrap();

    let app = Router::new()
        .route("/interact", post(handle_interaction))
        .into_make_service();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    eprintln!("serving on {addr}");
    Server::bind(&addr).serve(app).await.unwrap();
}
