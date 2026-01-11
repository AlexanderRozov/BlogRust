pub mod config;
pub mod db;
pub mod models;
pub mod routes;
pub mod templates;

pub use config::Config;
pub use db::connect;
pub use models::*;

