pub mod user {
    use crate::users::User;
    use axum::{extract::Path, http::StatusCode, Extension, Json};
    use serde::{Deserialize, Serialize};
    use sqlx::SqlitePool;

    #[derive(Serialize, Deserialize)]
    pub struct RequestBody {
        name: String,
    }

    pub async fn get_all(
        Extension(pool): Extension<SqlitePool>,
    ) -> Result<Json<Vec<User>>, StatusCode> {
        Ok(Json(
            User::get_all(&pool)
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?,
        ))
    }

    pub async fn get(
        Extension(pool): Extension<SqlitePool>,
        Path(id): Path<i64>,
    ) -> Result<Json<User>, StatusCode> {
        Ok(Json(
            User::get_by_id(&pool, id)
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?,
        ))
    }

    pub async fn create(
        Extension(pool): Extension<SqlitePool>,
        Json(body): Json<RequestBody>,
    ) -> Result<(), StatusCode> {
        User::create(&pool, &body.name)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)
    }

    pub async fn update(
        Extension(pool): Extension<SqlitePool>,
        Path(id): Path<i64>,
        Json(body): Json<RequestBody>,
    ) -> Result<(), StatusCode> {
        User::update(&pool, id, &body.name)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)
    }

    pub async fn delete(
        Extension(pool): Extension<SqlitePool>,
        Path(id): Path<i64>,
    ) -> Result<(), StatusCode> {
        User::delete(&pool, id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)
    }
}

pub mod message {
    use crate::messages::Message;
    use axum::{extract::Path, http::StatusCode, Extension, Json};
    use serde::{Deserialize, Serialize};
    use sqlx::SqlitePool;

    #[derive(Serialize, Deserialize)]
    pub struct RequestBody {
        message: String,
    }

    pub async fn get(
        Path((sender_id, receiver_id)): Path<(i64, i64)>,
        Extension(pool): Extension<SqlitePool>,
    ) -> Result<Json<Vec<Message>>, StatusCode> {
        Ok(Json(
            Message::chat_history(&pool, sender_id, receiver_id)
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?,
        ))
    }

    pub async fn create(
        Path((sender_id, receiver_id)): Path<(i64, i64)>,
        Extension(pool): Extension<SqlitePool>,
        Json(body): Json<RequestBody>,
    ) -> Result<(), StatusCode> {
        Message::create(&pool, sender_id, receiver_id, &body.message)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)
    }
}
