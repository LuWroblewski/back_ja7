use axum::http::{self, header};
use axum::{routing::get, routing::post, Router};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

//use back_ja7::middlewares::jwt_auth::jwt_auth;
use back_ja7::routes::auth::login::login;
use back_ja7::routes::users::del_user::del_user;
use back_ja7::routes::users::get_all_users::get_all_users;
use back_ja7::routes::users::get_user::get_user;
use back_ja7::routes::users::post_user::post_user;
use back_ja7::routes::users::put_user::put_user;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_origin(Any)
        .expose_headers([header::CONTENT_LENGTH]);

    let app: Router = Router::new()
        .route("/users", get(get_all_users).post(post_user))
        .route("/users/:id", get(get_user).put(put_user).delete(del_user))
        .route("/auth", post(login))
        .layer(cors);

    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
