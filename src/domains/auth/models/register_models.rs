use crate::UserResponse;

#[derive(FromForm, Debug)]
pub struct UserRegister<'r> {
    pub email: &'r str,
    pub password: &'r str,
    pub name: &'r str,
}

#[derive(serde::Serialize, Debug)]
pub struct UserRegistrationResponse {
    pub token: Option<String>,
    pub message: String,
    pub user: Option<UserResponse>,
}
