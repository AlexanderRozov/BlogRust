use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateComment {
    pub author: String,
    pub content: String,
}

impl Comment {
    pub async fn find_by_post_id(pool: &sqlx::PgPool, post_id: Uuid) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Comment>(
            "SELECT id, post_id, author, content, created_at 
             FROM comments 
             WHERE post_id = $1 
             ORDER BY created_at ASC"
        )
        .bind(post_id)
        .fetch_all(pool)
        .await
    }

    pub async fn find_all(pool: &sqlx::PgPool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Comment>(
            "SELECT id, post_id, author, content, created_at 
             FROM comments 
             ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(pool: &sqlx::PgPool, post_id: Uuid, comment: CreateComment) -> sqlx::Result<Self> {
        sqlx::query_as::<_, Comment>(
            "INSERT INTO comments (post_id, author, content) 
             VALUES ($1, $2, $3) 
             RETURNING id, post_id, author, content, created_at"
        )
        .bind(post_id)
        .bind(&comment.author)
        .bind(&comment.content)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &sqlx::PgPool, id: Uuid) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM comments WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map(|_| ())
    }

    pub fn formatted_datetime(&self) -> String {
        self.created_at.format("%d.%m.%Y %H:%M").to_string()
    }

    pub fn iso_datetime(&self) -> String {
        self.created_at.format("%+").to_string()
    }

    pub fn formatted_content(&self) -> String {
        self.content.replace('\n', "<br>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_comment() -> Comment {
        Comment {
            id: Uuid::new_v4(),
            post_id: Uuid::new_v4(),
            author: "Test User".to_string(),
            content: "This is a test comment.".to_string(),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_formatted_datetime() {
        let comment = create_test_comment();
        let formatted = comment.formatted_datetime();
        assert!(!formatted.is_empty());
        assert!(formatted.contains(" "));
    }

    #[test]
    fn test_iso_datetime() {
        let comment = create_test_comment();
        let formatted = comment.iso_datetime();
        assert!(!formatted.is_empty());
    }

    #[test]
    fn test_formatted_content() {
        let comment = Comment {
            content: "Line 1\nLine 2".to_string(),
            ..create_test_comment()
        };
        let formatted = comment.formatted_content();
        assert!(formatted.contains("<br>"));
        assert!(!formatted.contains('\n'));
    }

    #[test]
    fn test_formatted_content_no_newlines() {
        let comment = Comment {
            content: "Single line comment".to_string(),
            ..create_test_comment()
        };
        let formatted = comment.formatted_content();
        assert_eq!(formatted, "Single line comment");
    }
}

