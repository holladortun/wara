use std::fmt;

use crate::libs::{config::Config, db::Database};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Database,
}

impl AppState {
    pub fn new(config: Config, db: Database) -> Self {
        Self { config, db }
    }
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("config", &"<redacted>")
            .field("db", &"ToastyDatabase")
            .finish()
    }
}
