use axum::{
    http::{HeaderValue, Method},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use tower_http::{
    services::{ServeDir, ServeFile},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let frontend = async {
        let app = Router::new().route_service("/", ServeFile::new("src/assets/index.html"))
                         .nest_service("/assets", ServeDir::new("src/assets"));
        serve(app, 3000).await;
    };

    let backend = async {
        let app = Router::new().route("/json", get(json));
        serve(app, 4000).await;
    };

    tokio::join!(frontend, backend);
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn html() -> impl IntoResponse {
    Html(include_str!("assets/index.html"))
}

async fn json() -> impl IntoResponse {
    Json(vec!["one", "two", "three"])
}