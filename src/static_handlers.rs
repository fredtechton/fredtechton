use axum::Router;
use axum::body::{boxed, Empty, Full};
use axum::extract::{Path};
use axum::http::{HeaderValue, Response};
use axum::response::{IntoResponse};
use axum::routing::get;
use include_dir::{include_dir, Dir};

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

pub fn router() -> Router {
    Router::new()
        .route("/static/*path", get(serve_file))
        .route("/book/*path", get(serve_book))
}