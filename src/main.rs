use rocket::{form::Form, http::Status, serde::json::Json};
use dotenv::dotenv;
use std::env;

#[macro_use] extern crate rocket;

#[derive(FromForm, Debug)]
struct UserRegister<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(serde::Serialize, Debug)]
struct UserRegistrationResponse {
    token: Option<String>,
    message: String,
}

#[derive(serde::Serialize, Debug)]
struct ConfigInfo {
    database_url: String,
    rabbitmq_url: String,
    rocket_address: String,
    rocket_port: String,
}

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world! 🌎"
}

#[get("/config")]
fn get_config() -> Json<ConfigInfo> {
    Json(ConfigInfo {
        database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string()),
        rabbitmq_url: env::var("RABBITMQ_URL").unwrap_or_else(|_| "not set".to_string()),
        rocket_address: env::var("ROCKET_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string()),
        rocket_port: env::var("ROCKET_PORT").unwrap_or_else(|_| "8000".to_string()),
    })
}

#[post("/register", data = "<user>")]
fn register(user: Form<UserRegister<'_>>) -> (Status, Json<UserRegistrationResponse>) {
    let response = UserRegistrationResponse {
        token: None,
        message: format!("Here is the login info: {} - {}", user.email, user.password)
    };
    (Status::Created, Json(response))
}

#[launch]
fn rocket() -> _ {
    // Carrega variáveis de ambiente do arquivo .env
    dotenv().ok();
    
    println!("🚀 Iniciando Courses CRUD API...");
    println!("📊 Database URL: {}", env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string()));
    println!("🐰 RabbitMQ URL: {}", env::var("RABBITMQ_URL").unwrap_or_else(|_| "not set".to_string()));
    
    rocket::build()
        .mount("/api/v1", routes![index, get_config])
        .mount("/api/v1/auth", routes![register])
}