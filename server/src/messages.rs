use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    id: i64,
    sender_id: i64,
    receiver_id: i64,
    message_text: String,
    sent_at: chrono::NaiveDateTime,
}

impl Message {
    fn new(
        id: i64,
        sender_id: i64,
        receiver_id: i64,
        message_text: &str,
        sent_at: chrono::NaiveDateTime,
    ) -> Self {
        Self {
            id,
            sender_id,
            receiver_id,
            message_text: message_text.to_string(),
            sent_at,
        }
    }

    pub async fn create(
        db: &SqlitePool,
        sender_id: i64,
        receiver_id: i64,
        message_text: &str,
    ) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            "INSERT INTO messages (sender_id, receiver_id, message_text) VALUES (?, ?, ?)",
            sender_id,
            receiver_id,
            message_text
        )
        .execute(db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_all(db: &SqlitePool) -> Result<Vec<Message>, sqlx::Error> {
        match sqlx::query!("SELECT * FROM messages").fetch_all(db).await {
            Ok(rows) => Ok(rows
                .iter()
                .map(|row| {
                    Message::new(
                        row.id,
                        row.sender_id,
                        row.receiver_id,
                        &row.message_text,
                        row.sent_at.unwrap(),
                    )
                })
                .collect()),
            Err(e) => Err(e),
        }
    }

    pub async fn find_by_sender_id(
        db: &SqlitePool,
        sender_id: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        match sqlx::query!("SELECT * FROM messages WHERE sender_id = ?", sender_id)
            .fetch_all(db)
            .await
        {
            Ok(rows) => Ok(rows
                .iter()
                .map(|row| {
                    Message::new(
                        row.id,
                        row.sender_id,
                        row.receiver_id,
                        &row.message_text,
                        row.sent_at.unwrap(),
                    )
                })
                .collect()),
            Err(e) => Err(e),
        }
    }

    pub async fn find_by_receiver_id(
        db: &SqlitePool,
        receiver_id: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        match sqlx::query!("SELECT * FROM messages WHERE receiver_id = ?", receiver_id)
            .fetch_all(db)
            .await
        {
            Ok(rows) => Ok(rows
                .iter()
                .map(|row| {
                    Message::new(
                        row.id,
                        row.sender_id,
                        row.receiver_id,
                        &row.message_text,
                        row.sent_at.unwrap(),
                    )
                })
                .collect()),
            Err(e) => Err(e),
        }
    }

    pub async fn find_from_sender_id_to_receiver_id(
        db: &SqlitePool,
        sender_id: i64,
        receiver_id: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        match sqlx::query!(
            "SELECT * FROM messages WHERE sender_id = ? AND receiver_id = ?",
            sender_id,
            receiver_id
        )
        .fetch_all(db)
        .await
        {
            Ok(rows) => Ok(rows
                .iter()
                .map(|row| {
                    Message::new(
                        row.id,
                        row.sender_id,
                        row.receiver_id,
                        &row.message_text,
                        row.sent_at.unwrap(),
                    )
                })
                .collect()),
            Err(e) => Err(e),
        }
    }

    pub async fn chat_history(
        db: &SqlitePool,
        user1: i64,
        user2: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        match sqlx::query!("SELECT * FROM messages WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1) ORDER BY sent_at ASC", user1, user2).fetch_all(db).await {
            Ok(rows) => Ok(rows.iter().filter_map(|row| {
                row.id.map(|id| Message::new(
                        id,
                        row.sender_id,
                        row.receiver_id,
                        &row.message_text,
                        row.sent_at.unwrap(),
                    ))
            }).collect()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_from_sender_id_to_receiver_id(
        db: &SqlitePool,
        sender_id: i64,
        receiver_id: i64,
    ) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            "DELETE FROM messages WHERE sender_id = ? AND receiver_id = ?",
            sender_id,
            receiver_id
        )
        .execute(db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_by_id(db: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        match sqlx::query!("DELETE FROM messages WHERE id = ?", id)
            .execute(db)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
