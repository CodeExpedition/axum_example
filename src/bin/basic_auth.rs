use axum::{Router, routing::{get}};
use axum_auth::AuthBasic;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", get(handle_login));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()   
}
// Authorization: <type> <credentials>
// 클라이언트 에서 header{Authorization: "Basic base64(userid:password")}
async fn handle_login(AuthBasic((id, password)): AuthBasic) -> String {
    if let Some(password) = password {
        format!("user id: {}, password: {}", id, password)
    } else {
        format!("user {} without password", id )
    }
}