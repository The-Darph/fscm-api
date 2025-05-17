use crate::settings::Settings;
use std::sync::Arc;
use arc_swap::ArcSwap;
use crate::db::{establish_pool, DbPool};

// #[derive(Clone)]
pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub db_pool: DbPool,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            db_pool: establish_pool(),
        })
    }
}
