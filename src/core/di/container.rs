use std::sync::Arc;
use crate::config::db::DbPool;
use crate::domains::auth::repositories::user_repository::UserRepository;
use crate::domains::auth::repositories::user_repository_imp::UserRepositoryImp;
// Implementação do Fairing para Rocket
use rocket::{fairing::{Fairing, Info, Kind}, Rocket, Build};

#[derive(Clone)]
pub struct Container {
    user_repository: Arc<dyn UserRepository>,
}

impl Container {
    pub fn new(pool: DbPool) -> Self {
        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserRepositoryImp { pool }
        );

        Self { user_repository }
    }
}

#[rocket::async_trait]
impl Fairing for Container {
    fn info(&self) -> Info {
        Info {
            name: "Dependency Injection Container",
            kind: Kind::Ignite | Kind::Request,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        Ok(rocket.manage(self.clone()))
    }
}

