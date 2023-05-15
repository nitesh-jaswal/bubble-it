use std::net::SocketAddr;

use axum::{routing::get, Extension, Router};
use sqlx::SqlitePool;

mod messages;
mod routes;
mod users;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite://./db.sqlite").await?;

    let user_route = Router::new()
        .route(
            "/user",
            get(routes::user::get_all).post(routes::user::create),
        )
        .route(
            "/user/:id",
            get(routes::user::get)
                .put(routes::user::update)
                .delete(routes::user::delete),
        );

    let message_route = Router::new().route(
        "/message/:sender_id/:receiver_id",
        get(routes::message::get).post(routes::message::create),
    );

    let app = Router::new()
        .merge(user_route)
        .merge(message_route)
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
