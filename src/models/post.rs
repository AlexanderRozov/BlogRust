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
        let mut result = String::new();
        let mut chars = title.chars().peekable();
        
        while let Some(c) = chars.next() {
            let replacement = match c {
                'а' => "a", 'б' => "b", 'в' => "v", 'г' => "g", 'д' => "d",
                'е' => "e", 'ё' => "e", 'ж' => "zh", 'з' => "z", 'и' => "i",
                'й' => "y", 'к' => "k", 'л' => "l", 'м' => "m", 'н' => "n",
                'о' => "o", 'п' => "p", 'р' => "r", 'с' => "s", 'т' => "t",
                'у' => "u", 'ф' => "f", 'х' => "h", 'ц' => "ts", 'ч' => "ch",
                'ш' => "sh", 'щ' => "sch", 'ъ' => "", 'ы' => "y", 'ь' => "",
                'э' => "e", 'ю' => "yu", 'я' => "ya",
                c if c.is_alphanumeric() => {
                    result.push(c.to_ascii_lowercase());
                    continue;
                }
                c if c == '-' || c == '_' => {
                    result.push(c);
                    continue;
                }
                _ => "",
            };
            result.push_str(replacement);
        }
        
        result
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
            .trim_matches('-')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_post() -> Post {
        Post {
            id: uuid::Uuid::new_v4(),
            title: "Test Post".to_string(),
            slug: "test-post".to_string(),
            content: "This is a test post content.".to_string(),
            published_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_formatted_date() {
        let post = create_test_post();
        let formatted = post.formatted_date();
        assert!(!formatted.is_empty());
        assert!(formatted.contains("."));
    }

    #[test]
    fn test_formatted_datetime() {
        let post = create_test_post();
        let formatted = post.formatted_datetime();
        assert!(!formatted.is_empty());
        assert!(formatted.contains(" "));
    }

    #[test]
    fn test_iso_datetime() {
        let post = create_test_post();
        let formatted = post.iso_datetime();
        assert!(!formatted.is_empty());
        assert!(formatted.contains("-"));
    }

    #[test]
    fn test_content_preview_short() {
        let post = Post {
            content: "Short".to_string(),
            ..create_test_post()
        };
        let preview = post.content_preview(100);
        assert_eq!(preview, "Short");
    }

    #[test]
    fn test_content_preview_long() {
        let post = Post {
            content: "This is a very long content that should be truncated".to_string(),
            ..create_test_post()
        };
        let preview = post.content_preview(20);
        assert!(preview.len() <= 23); // 20 + "..."
        assert!(preview.ends_with("..."));
    }

    #[test]
    fn test_formatted_content() {
        let post = Post {
            content: "Line 1\nLine 2\nLine 3".to_string(),
            ..create_test_post()
        };
        let formatted = post.formatted_content();
        assert!(formatted.contains("<br>"));
        assert!(!formatted.contains('\n'));
    }

    #[test]
    fn test_slugify_english() {
        let result = Post::slugify("Hello World");
        assert_eq!(result, "hello-world");
    }

    #[test]
    fn test_slugify_russian() {
        let result = Post::slugify("Привет Мир");
        assert_eq!(result, "privet-mir");
    }

    #[test]
    fn test_slugify_special_chars() {
        let result = Post::slugify("Test!@#$%^&*()Post");
        assert_eq!(result, "testpost");
    }

    #[test]
    fn test_slugify_multiple_spaces() {
        let result = Post::slugify("Test    Post");
        assert_eq!(result, "test-post");
    }

    #[test]
    fn test_slugify_trim_dashes() {
        let result = Post::slugify("---Test Post---");
        assert_eq!(result, "test-post");
    }

    #[test]
    fn test_slugify_empty() {
        let result = Post::slugify("");
        assert_eq!(result, "");
    }
}

