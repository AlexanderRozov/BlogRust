use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub slug: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePost {
    pub title: String,
    pub slug: String,
    pub content: String,
}

impl Post {
    pub async fn find_all(pool: &sqlx::PgPool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Post>(
            "SELECT id, title, slug, content, published_at, created_at, updated_at 
             FROM posts 
             ORDER BY published_at DESC"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_slug(pool: &sqlx::PgPool, slug: &str) -> sqlx::Result<Option<Self>> {
        sqlx::query_as::<_, Post>(
            "SELECT id, title, slug, content, published_at, created_at, updated_at 
             FROM posts 
             WHERE slug = $1"
        )
        .bind(slug)
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> sqlx::Result<Option<Self>> {
        sqlx::query_as::<_, Post>(
            "SELECT id, title, slug, content, published_at, created_at, updated_at 
             FROM posts 
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &sqlx::PgPool, post: CreatePost) -> sqlx::Result<Self> {
        sqlx::query_as::<_, Post>(
            "INSERT INTO posts (title, slug, content) 
             VALUES ($1, $2, $3) 
             RETURNING id, title, slug, content, published_at, created_at, updated_at"
        )
        .bind(&post.title)
        .bind(&post.slug)
        .bind(&post.content)
        .fetch_one(pool)
        .await
    }

    pub async fn update(pool: &sqlx::PgPool, id: Uuid, post: UpdatePost) -> sqlx::Result<Self> {
        sqlx::query_as::<_, Post>(
            "UPDATE posts 
             SET title = $1, slug = $2, content = $3, updated_at = NOW() 
             WHERE id = $4 
             RETURNING id, title, slug, content, published_at, created_at, updated_at"
        )
        .bind(&post.title)
        .bind(&post.slug)
        .bind(&post.content)
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &sqlx::PgPool, id: Uuid) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map(|_| ())
    }

    pub fn formatted_date(&self) -> String {
        self.published_at.format("%d.%m.%Y").to_string()
    }

    pub fn formatted_datetime(&self) -> String {
        self.published_at.format("%d.%m.%Y %H:%M").to_string()
    }

    pub fn iso_datetime(&self) -> String {
        self.published_at.format("%Y-%m-%d").to_string()
    }

    pub fn content_preview(&self, max_len: usize) -> String {
        if self.content.len() > max_len {
            format!("{}...", &self.content[..max_len])
        } else {
            self.content.clone()
        }
    }

    pub fn formatted_content(&self) -> String {
        self.content.replace('\n', "<br>")
    }

    pub fn slugify(title: &str) -> String {
        title
            .to_lowercase()
            .chars()
            .map(|c| match c {
                'а' => 'a', 'б' => 'b', 'в' => 'v', 'г' => 'g', 'д' => 'd',
                'е' => 'e', 'ё' => 'e', 'ж' => 'zh', 'з' => 'z', 'и' => 'i',
                'й' => 'y', 'к' => 'k', 'л' => 'l', 'м' => 'm', 'н' => 'n',
                'о' => 'o', 'п' => 'p', 'р' => 'r', 'с' => 's', 'т' => 't',
                'у' => 'u', 'ф' => 'f', 'х' => 'h', 'ц' => 'ts', 'ч' => 'ch',
                'ш' => 'sh', 'щ' => 'sch', 'ъ' => '', 'ы' => 'y', 'ь' => '',
                'э' => 'e', 'ю' => 'yu', 'я' => 'ya',
                _ => c,
            })
            .filter(|c| c.is_alphanumeric() || c == &'-' || c == &'_')
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
            .trim_matches('-')
            .to_string()
    }
}

