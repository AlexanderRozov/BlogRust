use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    Form,
};
use askama::Template;
use crate::models::{Post, Comment, CreateComment};
use crate::templates::{IndexTemplate, PostTemplate};

pub fn router() -> axum::Router<sqlx::PgPool> {
    axum::Router::new()
        .route("/", axum::routing::get(index))
        .route("/post/:slug", axum::routing::get(post_detail))
        .route("/post/:slug/comment", axum::routing::post(create_comment))
}

async fn index(State(pool): State<sqlx::PgPool>) -> Result<Html<String>, String> {
    let posts = Post::find_all(&pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    let template = IndexTemplate { posts };
    template
        .render()
        .map(Html)
        .map_err(|e| format!("Template error: {}", e))
}

async fn post_detail(
    State(pool): State<sqlx::PgPool>,
    Path(slug): Path<String>,
) -> Result<Html<String>, String> {
    let post = Post::find_by_slug(&pool, &slug)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Post not found".to_string())?;

    let comments = Comment::find_by_post_id(&pool, post.id)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    let template = PostTemplate {
        post: post.clone(),
        comments,
        slug: post.slug.clone(),
    };
    template
        .render()
        .map(Html)
        .map_err(|e| format!("Template error: {}", e))
}

async fn create_comment(
    State(pool): State<sqlx::PgPool>,
    Path(slug): Path<String>,
    Form(comment): Form<CreateComment>,
) -> Result<Redirect, String> {
    let post = Post::find_by_slug(&pool, &slug)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Post not found".to_string())?;

    Comment::create(&pool, post.id, comment)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(Redirect::to(&format!("/post/{}", slug)))
}

