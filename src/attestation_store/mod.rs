pub mod redis;

use async_trait::async_trait;

#[async_trait]
pub trait AttestationStore {
    async fn set_attestation(&self, id: &str, origin: &str) -> Result<()>;
    async fn get_attestation(&self, id: &str) -> Result<String>;
}

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;
