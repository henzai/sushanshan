use axum::routing::{get, post};
use axum::Router;
use handler::handle_su_shan_shan;
mod handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(handle_su_shan_shan));

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
