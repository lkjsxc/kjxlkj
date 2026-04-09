use crate::config::Config;
use crate::error::AppError;
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::{config::Builder, Client};

#[derive(Clone)]
pub struct Storage {
    bucket: String,
    client: Client,
}

impl Storage {
    pub async fn from_config(config: &Config) -> Result<Self, AppError> {
        let shared = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(config.s3_region.clone()))
            .credentials_provider(Credentials::new(
                config.s3_access_key.clone(),
                config.s3_secret_key.clone(),
                None,
                None,
                "kjxlkj-static",
            ))
            .endpoint_url(config.s3_endpoint.clone())
            .load()
            .await;
        let conf = Builder::from(&shared)
            .force_path_style(config.s3_path_style)
            .build();
        let storage = Self {
            bucket: config.s3_bucket.clone(),
            client: Client::from_conf(conf),
        };
        storage.ensure_bucket().await?;
        Ok(storage)
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    async fn ensure_bucket(&self) -> Result<(), AppError> {
        if self
            .client
            .head_bucket()
            .bucket(&self.bucket)
            .send()
            .await
            .is_ok()
        {
            return Ok(());
        }
        self.client
            .create_bucket()
            .bucket(&self.bucket)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| AppError::StorageError(format!("bucket init failed: {e}")))
    }
}
