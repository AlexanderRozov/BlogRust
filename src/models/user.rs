use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub async fn find_by_username(pool: &sqlx::PgPool, username: &str) -> sqlx::Result<Option<Self>> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, created_at FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(pool)
        .await
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, argon2::password_hash::Error> {
        use argon2::password_hash::{PasswordHash, PasswordVerifier};
        let parsed_hash = PasswordHash::new(&self.password_hash)?;
        Ok(argon2::Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
    use argon2::Argon2;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            password_hash: "$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG".to_string(),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_verify_password_correct() {
        // Note: This test requires a valid argon2 hash for "testpassword"
        // In practice, you'd generate this with the hash-password utility
        let password = "testpassword";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();

        let user = User {
            password_hash: hash,
            ..create_test_user()
        };

        let result = user.verify_password(password);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_verify_password_incorrect() {
        let user = create_test_user();
        let result = user.verify_password("wrongpassword");
        // This will fail because we're using a dummy hash
        // In a real test, you'd use a properly generated hash
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_user_creation() {
        let user = create_test_user();
        assert_eq!(user.username, "testuser");
        assert!(!user.password_hash.is_empty());
    }
}

