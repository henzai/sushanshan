use axum::routing::{get, post};
use axum::Router;
use sushanshan::handler::interaction::{handle_interaction, translate_to_japanese};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:text", get(translate_to_japanese))
        .route("/", post(handle_interaction))
        .route("/sushanshan", post(handle_interaction));

    let addr = ([0, 0, 0, 0], 8080).into();
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
