use axum::{Router, routing::{get}};
use axum_auth::AuthBearer;
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", get(handle_login));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()   
}
// Authorization: <type> <credentials> -> 쿠기를 통해 보냄
// 클라이언트 에서 header{Authorization: "Bearer token}
async fn handle_login(AuthBearer(token): AuthBearer) -> String {
    format!("user token: {}", token)
}