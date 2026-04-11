use crate::config::Config;
use crate::error::AppError;
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::{
    config::{Builder, RequestChecksumCalculation},
    primitives::ByteStream,
    Client,
};
use std::path::Path;
use tokio::time::{sleep, Duration};
use tracing::warn;

#[derive(Clone)]
pub struct Storage {
    bucket: String,
    client: Client,
}

pub struct StoredObject {
    pub body: Vec<u8>,
    pub content_length: i64,
    pub content_range: Option<String>,
}

impl Storage {
    pub async fn from_config(config: &Config) -> Result<Self, AppError> {
        let shared = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(config.seaweedfs_s3_region.clone()))
            .credentials_provider(Credentials::new(
                config.seaweedfs_s3_access_key.clone(),
                config.seaweedfs_s3_secret_key.clone(),
                None,
                None,
                "kjxlkj-static",
            ))
            .endpoint_url(config.seaweedfs_s3_endpoint.clone())
            .load()
            .await;
        let conf = Builder::from(&shared)
            .force_path_style(config.seaweedfs_s3_path_style)
            .request_checksum_calculation(RequestChecksumCalculation::WhenRequired)
            .build();
        let storage = Self {
            bucket: config.seaweedfs_s3_bucket.clone(),
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

    pub async fn put_object(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: &str,
    ) -> Result<(), AppError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .body(ByteStream::from(bytes))
            .send()
            .await
            .map(|_| ())
            .map_err(|e| AppError::StorageError(format!("object upload failed: {e}")))
    }

    pub async fn put_file(
        &self,
        key: &str,
        path: impl AsRef<Path>,
        content_type: &str,
    ) -> Result<(), AppError> {
        let body = ByteStream::from_path(path.as_ref())
            .await
            .map_err(|e| AppError::StorageError(format!("object file open failed: {e}")))?;
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .body(body)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| AppError::StorageError(format!("object upload failed: {e}")))
    }

    pub async fn get_object(
        &self,
        key: &str,
        range: Option<&str>,
    ) -> Result<StoredObject, AppError> {
        let mut request = self.client.get_object().bucket(&self.bucket).key(key);
        if let Some(range) = range {
            request = request.range(range);
        }
        let response = request
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("object fetch failed: {e}")))?;
        let body = response
            .body
            .collect()
            .await
            .map_err(|e| AppError::StorageError(format!("object stream failed: {e}")))?
            .into_bytes()
            .to_vec();
        Ok(StoredObject {
            body,
            content_length: response.content_length.unwrap_or_default(),
            content_range: response.content_range,
        })
    }

    pub async fn delete_object(&self, key: &str) -> Result<(), AppError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| AppError::StorageError(format!("object delete failed: {e}")))
    }

    async fn ensure_bucket(&self) -> Result<(), AppError> {
        let mut last_error = None;
        for attempt in 1..=10 {
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
            match self
                .client
                .create_bucket()
                .bucket(&self.bucket)
                .send()
                .await
            {
                Ok(_) => return Ok(()),
                Err(error) => {
                    last_error = Some(error.to_string());
                    if attempt == 10 {
                        break;
                    }
                    warn!(
                        bucket = %self.bucket,
                        attempt,
                        "object storage bucket not ready yet; retrying initialization"
                    );
                    sleep(Duration::from_millis(500 * attempt as u64)).await;
                }
            }
        }
        Err(AppError::StorageError(format!(
            "bucket init failed after retries: {}",
            last_error.unwrap_or_else(|| "unknown storage error".to_string())
        )))
    }
}
