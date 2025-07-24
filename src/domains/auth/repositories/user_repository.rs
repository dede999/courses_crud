use std::error::Error;
use uuid::Uuid;
use async_trait::async_trait;
use crate::domains::auth::models::user_models::{NewUser, UserResponse};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<UserResponse>, Box<dyn Error>>;
    async fn create_user(
        &self,
        new_user_data: NewUser
    ) -> Result<UserResponse, Box<dyn Error>>;
    async fn update_user(
        &self,
        user_id: Uuid,
        new_email: &str,
        new_name: &str,
    ) -> Result<UserResponse, Box<dyn Error>>;
    async fn update_password(
        &self,
        id: Uuid,
        new_password: &str
    ) -> Result<UserResponse, Box<dyn Error>>;
    async fn delete_user(&self, id: Uuid) -> Result<i32, Box<dyn Error>>;
}