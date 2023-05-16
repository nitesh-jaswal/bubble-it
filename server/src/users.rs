use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i64,
    name: String,
}

impl User {
    fn new(id: i64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub async fn create(db: &SqlitePool, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO users (name) VALUES (?)", name)
            .execute(db)
            .await
            .map(|_| ())
    }

    pub async fn get_by_id(db: &SqlitePool, id: i64) -> Result<User, sqlx::Error> {
        sqlx::query!("SELECT * FROM users WHERE id = ?", id)
            .fetch_one(db)
            .await
            .map(|row| User::new(row.id, &row.name))
    }

    pub async fn get_by_name(db: &SqlitePool, name: &str) -> Result<User, sqlx::Error> {
        sqlx::query!("SELECT * FROM users WHERE name = ?", name)
            .fetch_one(db)
            .await
            .map(|row| User::new(row.id, &row.name))
    }

    pub async fn get_all(db: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query!("SELECT * FROM users")
            .fetch_all(db)
            .await
            .map(|rows| {
                rows.iter()
                    .map(|row| User::new(row.id, &row.name))
                    .collect()
            })
    }

    pub async fn delete(db: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(db)
            .await
            .map(|_| ())
    }

    pub async fn update(db: &SqlitePool, id: i64, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE users SET name = ? WHERE id = ?", name, id)
            .execute(db)
            .await
            .map(|_| ())
    }
}
