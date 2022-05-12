
use axum::extract::{Path, Form};
use axum::http::{HeaderValue, StatusCode, Response};
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, Router, post};
use axum::Server;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


mod static_handlers;

#[derive(Debug, Deserialize)]
struct Invite {
    email: Option<String>
}

async fn join(Form(invite): Form<Invite>) -> Result<String, (StatusCode, String)> {
    println!("The invite is {:?}", invite);
    Err((StatusCode::UNPROCESSABLE_ENTITY, "Unable to get email".into()))
}


#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "fredtechton=debug,tower_http=debug".into())
    )).with(tracing_subscriber::fmt::layer()).init();
    let router = Router::new()
        .route("/", get(|| async { Redirect::to("/book/index.html")}))
        .route("/join", post(join))
        .merge(static_handlers::router())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    tracing::debug!("Started listening for requests");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
