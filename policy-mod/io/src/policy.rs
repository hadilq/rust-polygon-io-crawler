use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use thiserror::Error;
use tokio::time::Instant;

/// The data that is needed to interact with the Policy modules.
pub struct Data {
    pub last_requests_time: Arc<Mutex<VecDeque<Instant>>>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            last_requests_time: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// The API of Policy, which defines all the functionality of Policy modules.
#[async_trait]
pub trait Api {
    async fn handle_request_rate(&self, data: &mut Data) -> Result<()>;
}

/// All possible errors of Policy modules.
#[derive(Debug, Error)]
pub enum Error {
    #[error("policy cannot lock the thread! {0}")]
    GuardError(String),
}
