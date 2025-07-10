use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub name: String,
}

// #[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
// #[diesel(table_name = crate::schema::courses)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Course {
//     pub id: i32,
//     pub title: String,
//     pub description: Option<String>,
//     pub instructor_id: i32,
//     pub price: f64,
//     pub is_active: bool,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

// #[derive(Debug, Serialize, Deserialize, Insertable)]
// #[diesel(table_name = crate::schema::courses)]
// pub struct NewCourse {
//     pub title: String,
//     pub description: Option<String>,
//     pub instructor_id: i32,
//     pub price: f64,
//     pub is_active: bool,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
// #[diesel(table_name = crate::schema::enrollments)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Enrollment {
//     pub id: i32,
//     pub user_id: i32,
//     pub course_id: i32,
//     pub enrolled_at: DateTime<Utc>,
//     pub status: String,
// }

// #[derive(Debug, Serialize, Deserialize, Insertable)]
// #[diesel(table_name = crate::schema::enrollments)]
// pub struct NewEnrollment {
//     pub user_id: i32,
//     pub course_id: i32,
//     pub status: String,
// }

// DTOs para API
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseResponse {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub instructor_id: i32,
    pub price: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrollmentResponse {
    pub id: i32,
    pub user_id: i32,
    pub course_id: i32,
    pub enrolled_at: DateTime<Utc>,
    pub status: String,
} 