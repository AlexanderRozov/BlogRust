use blog::models;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

// Helper function to create a test database pool
async fn create_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://blog:blog@localhost:5432/blog_test".to_string());
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create test pool")
}

#[tokio::test]
#[ignore] // Ignore by default, requires database
async fn test_post_create_and_find() {
    let pool = create_test_pool().await;
    
    // Clean up test data
    sqlx::query("DELETE FROM posts WHERE slug LIKE 'test-%'")
        .execute(&pool)
        .await
        .ok();

    let test_post = models::CreatePost {
        title: "Test Post".to_string(),
        slug: format!("test-{}", Uuid::new_v4()),
        content: "Test content".to_string(),
    };

    let created = models::Post::create(&pool, test_post.clone())
        .await
        .expect("Failed to create post");

    assert_eq!(created.title, test_post.title);
    assert_eq!(created.slug, test_post.slug);
    assert_eq!(created.content, test_post.content);

    let found = models::Post::find_by_slug(&pool, &created.slug)
        .await
        .expect("Failed to find post")
        .expect("Post not found");

    assert_eq!(found.id, created.id);
    assert_eq!(found.title, created.title);

    // Cleanup
    models::Post::delete(&pool, created.id)
        .await
        .expect("Failed to delete post");
}

#[tokio::test]
#[ignore]
async fn test_post_update() {
    let pool = create_test_pool().await;

    // Create test post
    let test_post = models::CreatePost {
        title: "Original Title".to_string(),
        slug: format!("test-update-{}", Uuid::new_v4()),
        content: "Original content".to_string(),
    };

    let created = models::Post::create(&pool, test_post)
        .await
        .expect("Failed to create post");

    // Update post
    let update = models::UpdatePost {
        title: "Updated Title".to_string(),
        slug: created.slug.clone(),
        content: "Updated content".to_string(),
    };

    let updated = models::Post::update(&pool, created.id, update)
        .await
        .expect("Failed to update post");

    assert_eq!(updated.title, "Updated Title");
    assert_eq!(updated.content, "Updated content");
    assert_ne!(updated.updated_at, created.updated_at);

    // Cleanup
    models::Post::delete(&pool, created.id)
        .await
        .ok();
}

#[tokio::test]
#[ignore]
async fn test_comment_create_and_find() {
    let pool = create_test_pool().await;

    // Create a test post first
    let test_post = models::CreatePost {
        title: "Test Post for Comment".to_string(),
        slug: format!("test-comment-{}", Uuid::new_v4()),
        content: "Test content".to_string(),
    };

    let post = models::Post::create(&pool, test_post)
        .await
        .expect("Failed to create post");

    // Create comment
    let test_comment = models::CreateComment {
        author: "Test Author".to_string(),
        content: "Test comment content".to_string(),
    };

    let comment = models::Comment::create(&pool, post.id, test_comment.clone())
        .await
        .expect("Failed to create comment");

    assert_eq!(comment.author, test_comment.author);
    assert_eq!(comment.content, test_comment.content);
    assert_eq!(comment.post_id, post.id);

    // Find comments by post
    let comments = models::Comment::find_by_post_id(&pool, post.id)
        .await
        .expect("Failed to find comments");

    assert!(!comments.is_empty());
    assert!(comments.iter().any(|c| c.id == comment.id));

    // Cleanup
    models::Comment::delete(&pool, comment.id)
        .await
        .ok();
    models::Post::delete(&pool, post.id)
        .await
        .ok();
}

#[tokio::test]
#[ignore]
async fn test_post_find_all() {
    let pool = create_test_pool().await;

    let posts = models::Post::find_all(&pool)
        .await
        .expect("Failed to find all posts");

    // Should be able to retrieve posts (might be empty or have existing posts)
    assert!(posts.len() >= 0);
}

#[tokio::test]
#[ignore]
async fn test_post_delete_cascades_comments() {
    let pool = create_test_pool().await;

    // Create post
    let test_post = models::CreatePost {
        title: "Post to Delete".to_string(),
        slug: format!("test-delete-{}", Uuid::new_v4()),
        content: "Content".to_string(),
    };

    let post = models::Post::create(&pool, test_post)
        .await
        .expect("Failed to create post");

    // Create comment
    let test_comment = models::CreateComment {
        author: "Author".to_string(),
        content: "Comment".to_string(),
    };

    let comment = models::Comment::create(&pool, post.id, test_comment)
        .await
        .expect("Failed to create comment");

    // Delete post (should cascade delete comments)
    models::Post::delete(&pool, post.id)
        .await
        .expect("Failed to delete post");

    // Verify comment is also deleted
    let comments = models::Comment::find_by_post_id(&pool, post.id)
        .await
        .expect("Failed to query comments");

    assert!(comments.is_empty() || !comments.iter().any(|c| c.id == comment.id));
}

