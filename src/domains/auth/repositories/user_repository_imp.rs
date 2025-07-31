use std::error::Error;
use crate::config::db::DbPool;
use crate::domains::auth::models::user_models::{NewUser, User, UserResponse};
use crate::domains::auth::repositories::user_repository::UserRepository;
use diesel::prelude::*;
use uuid::Uuid;
use async_trait::async_trait;
use crate::infrastructure::traits::model_response::ModelResponse;

pub struct UserRepositoryImp {
    pub pool: DbPool
}

#[async_trait]
impl UserRepository for UserRepositoryImp {
    async fn find_by_email(&self, given_email: &str) -> Result<Option<UserResponse>, Box<dyn Error>> {
        let mut conn = self.pool.get()
            .expect("Failed to get DB connection");
        use crate::schema::users::dsl::*;
        let result = users.filter(email.eq(given_email))
            .select(User::as_select())
            .first::<User>(&mut *conn)
            .optional()?;
        Ok(result.map(|user| user.to_response()))
    }

    async fn create_user(&self, new_user: NewUser) -> Result<UserResponse, Box<dyn Error>> {
        let mut conn = self.pool.get()
            .expect("Failed to get DB connection");
        use crate::schema::users::dsl::*;
        use crate::domains::auth::models::user_models::NewUser;

        let pwd_hash = bcrypt::hash(new_user.password_hash, bcrypt::DEFAULT_COST)
            .map_err(|b| format!("Failed to hash password: {}", b))?;
        let new_user = NewUser {
            email: new_user.email,
            password_hash: pwd_hash,
            name: new_user.name,
        };

        let created_user = diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&mut *conn)?;

        Ok(created_user.to_response())
    }

    async fn update_user(
        &self, user_id: Uuid, new_email: &str, new_name: &str,
    ) -> Result<UserResponse, Box<dyn Error>> {
        let mut conn = self.pool.get()
            .expect("Failed to get DB connection");
        use crate::schema::users::dsl::*;
        let for_update = users.filter(id.eq(user_id));

        let updated_user = diesel::update(for_update)
            .set((
                email.eq(new_email),
                name.eq(new_name),
            ))
            .get_result::<User>(&mut *conn)?;

        Ok(updated_user.to_response())
    }

    async fn update_password(
        &self, user_id: Uuid, new_password: &str
    ) -> Result<UserResponse, Box<dyn Error>> {
        let mut conn = self.pool.get()
            .expect("Failed to get DB connection");
        use crate::schema::users::dsl::*;
        let for_update = users.filter(id.eq(user_id));

        let pwd_hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)
            .map_err(|b| format!("Failed to hash password: {}", b))?;

        let updated_user = diesel::update(for_update)
            .set(password_hash.eq(pwd_hash))
            .get_result::<User>(&mut *conn)?;

        Ok(updated_user.to_response())
    }

    async fn delete_user(
        &self, deleted_id: Uuid
    ) -> Result<i32, Box<dyn Error>> {
        let mut conn = self.pool.get()
            .expect("Failed to get DB connection");
        use crate::schema::users::dsl::*;
        let for_deletion = users.filter(id.eq(deleted_id));
        let deleted_count = diesel::delete(for_deletion)
            .execute(&mut *conn)?;
        Ok(deleted_count as i32)
    }
}
