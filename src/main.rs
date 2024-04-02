use axum::{routing::get, Router};
use tokio::net::TcpListener;

use back_ja7::routes::users::getuser::get_user;
use back_ja7::routes::users::postuser::post_user;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/usuarios", get(get_user).post(post_user));

    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
