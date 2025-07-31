use rocket::{form::Form, http::Status, serde::json::Json, State};
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use config::db;

mod schema;
mod config;
mod domains;
mod core;
mod infrastructure;

use config::db::DbPool;
use crate::domains::auth::models::user_models::*;
use crate::domains::auth::models::register_models::*;

#[macro_use] extern crate rocket;


#[get("/hello")]
fn index() -> &'static str {
    "Hello, world! 🌎"
}


#[post("/register", data = "<user>")]
fn register(user: Form<UserRegister<'_>>, pool: &State<DbPool>) -> (Status, Json<UserRegistrationResponse>) {
    use schema::users;
    
    let conn = &mut pool.get().expect("Failed to get DB connection");
    
    // Hash da senha
    let password_hash = bcrypt::hash(user.password.as_bytes(), bcrypt::DEFAULT_COST)
        .unwrap_or_else(|_| "".to_string());
    
    let new_user = NewUser {
        email: user.email.to_string(),
        password_hash,
        name: user.name.to_string(),
    };
    
    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn) {
            Ok(created_user) => {
                let user_response = UserResponse {
                    id: created_user.id,
                    email: created_user.email,
                    name: created_user.name,
                    created_at: created_user.created_at,
                };
                
                (Status::Created, Json(UserRegistrationResponse {
                    token: None,
                    message: "Usuário criado com sucesso!".to_string(),
                    user: Some(user_response),
                }))
            },
            Err(e) => {
                (Status::BadRequest, Json(UserRegistrationResponse {
                    token: None,
                    message: format!("Erro ao criar usuário: {}", e),
                    user: None,
                }))
            }
        }
}

#[get("/users")]
fn get_users(pool: &State<DbPool>) -> Json<Vec<UserResponse>> {
    use schema::users;
    
    let conn = &mut pool.get().expect("Failed to get DB connection");
    
    let results = users::table
        .select(User::as_select())
        .load(conn)
        .expect("Error loading users");
    
    let user_responses: Vec<UserResponse> = results.into_iter().map(|user| UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        created_at: user.created_at,
    }).collect();
    
    Json(user_responses)
}

#[launch]
fn rocket() -> _ {
    // Carrega variáveis de ambiente do arquivo .env
    dotenv().ok();
    
    println!("🚀 Iniciando Courses CRUD API com Diesel...");
    println!("📊 Database URL: {}", env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string()));
    println!("🐰 RabbitMQ URL: {}", env::var("RABBITMQ_URL").unwrap_or_else(|_| "not set".to_string()));
    
    // Cria o pool de conexões
    let pool = db::create_pool();
    let container = core::di::container::Container::new(pool.clone());
    
    rocket::build()
        .attach(container)
        .mount("/api/v1", routes![index])
        .mount("/api/v1/auth", routes![register, get_users])
}