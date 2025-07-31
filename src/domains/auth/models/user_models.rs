use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::infrastructure::traits::model_response::ModelResponse;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ModelResponse for  User {
    type Response<'a> = UserResponse where Self: 'a;
    fn to_response(&self) -> Self::Response<'_> {
        UserResponse {
            id: self.id,
            email: self.email.clone(),
            name: self.name.clone(),
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub name: String,
}

// DTOs para API
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
