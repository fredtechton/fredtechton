
use axum::body::boxed;
use axum::body::Empty;
use axum::body::Full;
use axum::extract::{Path, Form};
use axum::http::Response;
use axum::http::{HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::{get, Router, post};
use axum::{Server};
use include_dir::{include_dir, Dir};
use serde::{Deserialize};


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
    email: Option<String>
}

async fn join(Form(invite): Form<Invite>) -> Result<String, (StatusCode, String)> {
    println!("The invite is {:?}", invite);
    Err((StatusCode::UNPROCESSABLE_ENTITY, "Unable to get email".into()))
}


#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(|| async { Redirect::to("/book/index.html")}))
        .route("/static/*path", get(serve_file))
        .route("/book/*path", get(serve_book))
        .route("/join", post(join));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
