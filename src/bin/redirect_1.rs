use axum::{Router, routing::{get, post}};
use axum::response::{IntoResponse, Html};
use axum::Form;
use serde::Deserialize;

//  form  요청시 특정 데이터와 같이 사용자를 다른리소스로 redirect 하고 싶을때 
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/redirectme", get(make_redirect_html))
        .route("/process_redirect", post(process_redirect));
    
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
    .unwrap()
}

#[derive(Deserialize, Debug)]
struct Data {
    data: String,
}

async fn make_redirect_html() -> impl IntoResponse {
    let html = include_str!("../../views/redirect_html.html");
    Html(html)
}

async fn process_redirect(Form(d): Form<Data>) {
    println!("data{}", d.data);
}