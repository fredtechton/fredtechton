use std::fmt;
use std::fmt::Display;
use std::io::Error;
use std::time::Duration;

use axum::body::Body;
use axum::body::boxed;
use axum::body::Empty;
use axum::body::Full;
use axum::extract::{Path, Form};
use axum::http::Response;
use axum::http::{HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::{get, Router, post};
use axum::error_handling::HandleError;
use axum::{Server, BoxError};
use axum::error_handling::HandleErrorLayer;
use include_dir::{include_dir, Dir};
use serde::{Serialize, Deserialize};
use tower::{self, ServiceBuilder};
use std::env;

use anyhow::{Result, Context};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");
static BOOK_DIR: Dir<'_> = include_dir!("$OUT_DIR");

async fn serve_file(Path(mut path): Path<String>) -> impl IntoResponse {
    path = path.trim_start_matches('/').to_string();
    let mime = mime_guess::from_path(&path).first_or_text_plain();
    match STATIC_DIR.get_file(&path) {
        None => Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .header(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_str(mime.as_ref()).unwrap(),
            )
            .body(boxed(Full::from(file.contents())))
            .unwrap(),
    }
}
async fn serve_book(Path(mut path): Path<String>) -> impl IntoResponse {
    path = path.trim_start_matches('/').to_string();
    let mime = mime_guess::from_path(&path).first_or_text_plain();
    match BOOK_DIR.get_file(&path) {
        None => Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .header(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_str(mime.as_ref()).unwrap(),
            )
            .body(boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

#[derive(Debug, Deserialize)]
struct Invite {
    email: String
}

#[derive(Debug)]
struct MyError {
  msg: String
}
impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.msg)
    }
}
async fn join(Form(invite): Form<Invite>) -> Result<String> {
    println!("The invite is {:?}", invite);
    // env::var_os("Something").ok_or(Err)
    Err(anyhow::anyhow!("Anyhow error"))
}



async fn handle_anyhow_errors(err: anyhow::Error) -> (StatusCode, String) {
    {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(|| async { Redirect::to("/book/index.html")}))
        .route("/static/*path", get(serve_file))
        .route("/book/*path", get(serve_book))
        // .route("/join", post(join))
        .route("/join", HandleError::new(join, handle_anyhow_errors));
        // .layer(
        //     ServiceBuilder::new()
        //         .layer(HandleErrorLayer::new(handle_anyhow_errors))
        //         .timeout(Duration::from_secs(10))
        //         .into_inner()
        // );

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
