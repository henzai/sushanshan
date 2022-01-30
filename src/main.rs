use axum::routing::{get, post};
use axum::Router;
use handler::su_shan_shan;

use handler::trans;
mod handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/:text", get(trans))
        .route("/", post(su_shan_shan));

    let addr = ([0, 0, 0, 0], 8080).into();
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
