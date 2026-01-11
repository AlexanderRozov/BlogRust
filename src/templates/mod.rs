use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub posts: Vec<crate::models::Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub post: crate::models::Post,
    pub comments: Vec<crate::models::Comment>,
    pub slug: String,
}

#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct AdminLoginTemplate {}

#[derive(Template)]
#[template(path = "admin/dashboard.html")]
pub struct AdminDashboardTemplate {
    pub posts: Vec<crate::models::Post>,
    pub comments: Vec<crate::models::Comment>,
}

