use axum::{routing::get, Router};
use tokio::net::TcpListener;

use back_ja7::routes::users::del_user::del_user;
use back_ja7::routes::users::get_user::get_user;
use back_ja7::routes::users::post_user::post_user;
use back_ja7::routes::users::put_user::put_user;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route(
        "/users",
        get(get_user).post(post_user).delete(del_user).put(put_user),
    );

    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
