use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    Form,
};
use askama::Template;
use tower_sessions::Session;
use uuid::Uuid;
use serde::Deserialize;
use crate::models::{Post, Comment, CreatePost, UpdatePost};
use crate::templates::{AdminLoginTemplate, AdminDashboardTemplate};

const SESSION_KEY: &str = "user_id";

pub fn router() -> axum::Router<sqlx::PgPool> {
    axum::Router::new()
        .route("/admin/login", axum::routing::get(login_page).post(login))
        .route("/admin/logout", axum::routing::post(logout))
        .route("/admin", axum::routing::get(dashboard))
        .route("/admin/posts", axum::routing::post(create_post))
        .route("/admin/posts/:id", axum::routing::post(update_post))
        .route("/admin/posts/:id/delete", axum::routing::post(delete_post))
        .route("/admin/comments/:id/delete", axum::routing::post(delete_comment))
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

async fn login_page() -> Html<String> {
    let template = AdminLoginTemplate {};
    Html(template.render().unwrap_or_else(|e| format!("Template error: {}", e)))
}

async fn login(
    State(pool): State<sqlx::PgPool>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, String> {
    let user = crate::models::User::find_by_username(&pool, &form.username)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Invalid credentials".to_string())?;

    user.verify_password(&form.password)
        .map_err(|_| "Invalid credentials".to_string())?
        .then_some(())
        .ok_or_else(|| "Invalid credentials".to_string())?;

    session
        .insert(SESSION_KEY, user.id.to_string())
        .map_err(|e| format!("Session error: {}", e))?;

    Ok(Redirect::to("/admin"))
}

async fn logout(session: Session) -> Result<Redirect, String> {
    session.delete();
    Ok(Redirect::to("/admin/login"))
}

/*
async fn require_auth(session: &Session) -> Result<(), Redirect> {
    let user_id: Option<String> = session.get(SESSION_KEY).ok().flatten();
    if user_id.is_none() {
        return Err(Redirect::to("/admin/login"));
    }
    Ok(())
} */

async fn require_auth(_session: &Session) -> Result<(), Redirect> {
    // TODO: Временно отключена проверка авторизации - все считаются админами
    // Позже нужно будет вернуть проверку авторизации
    Ok(())
}

#[derive(Deserialize)]
pub struct PostForm {
    title: String,
    slug: String,
    content: String,
}

async fn dashboard(
    State(pool): State<sqlx::PgPool>,
    session: Session,
) -> Result<Html<String>, Redirect> {
    require_auth(&session).await?;

    let posts = Post::find_all(&pool)
        .await
        .unwrap_or_default();
    
    let comments = Comment::find_all(&pool)
        .await
        .unwrap_or_default();

    let template = AdminDashboardTemplate { posts, comments };
    Ok(Html(template.render().unwrap_or_else(|e| format!("Template error: {}", e))))
}

async fn create_post(
    State(pool): State<sqlx::PgPool>,
    session: Session,
    Form(form): Form<PostForm>,
) -> Result<Redirect, Redirect> {
    require_auth(&session).await?;

    let post = CreatePost {
        title: form.title,
        slug: form.slug,
        content: form.content,
    };

    Post::create(&pool, post).await.ok();
    Ok(Redirect::to("/admin"))
}

async fn update_post(
    State(pool): State<sqlx::PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
    Form(form): Form<PostForm>,
) -> Result<Redirect, Redirect> {
    require_auth(&session).await?;

    let post = UpdatePost {
        title: form.title,
        slug: form.slug,
        content: form.content,
    };

    Post::update(&pool, id, post).await.ok();
    Ok(Redirect::to("/admin"))
}

async fn delete_post(
    State(pool): State<sqlx::PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<Redirect, Redirect> {
    require_auth(&session).await?;

    Post::delete(&pool, id).await.ok();
    Ok(Redirect::to("/admin"))
}

async fn delete_comment(
    State(pool): State<sqlx::PgPool>,
    session: Session,
    Path(id): Path<Uuid>,
) -> Result<Redirect, Redirect> {
    require_auth(&session).await?;

    Comment::delete(&pool, id).await.ok();
    Ok(Redirect::to("/admin"))
}

