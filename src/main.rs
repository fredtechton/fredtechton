use axum::Server;
use axum::body::Body;
use axum::body::Empty;
use axum::body::Full;
use axum::body::boxed;
use axum::extract::Path;
use axum::response::Response;
use axum::response::IntoResponse;
use axum::routing::{Router, get};
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

async fn serve_file(Path(mut path): Path<String>) -> impl IntoResponse {
    path = path.trim_start_matches('/').to_string();
    match STATIC_DIR.get_file(path) {
        None => Response::builder().status(axum::http::StatusCode::NOT_FOUND).body(boxed(Empty::new())).unwrap(),
        Some(file) => Response::builder().header("Conent-Type", "text/html").body(boxed(Full::from(file.contents()))).unwrap()
    }
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(|| async {"hello world"}))
    .route("/static/*path",get(serve_file));

    Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(router.into_make_service()).await.unwrap();
}
