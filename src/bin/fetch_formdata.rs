use axum::Router;
use axum::Form;
use axum::routing::{post};
use axum::response::{IntoResponse, Html};
use serde::{Deserialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/form", post(handler));
    
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()

}

#[derive(Deserialize, Debug)]
struct User {
    user_name: String,
    password: String,
}

async fn handler(Form(user): Form<User>) -> impl IntoResponse {
    println!("username:{} and password:{}", user.user_name, user.password);
    Html("<html><body><p>goodUserauth</p></body></html>")
} 