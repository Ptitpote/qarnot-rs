use crate::compute::client::ComputeClient;
use crate::compute::task::{InstancesOrRange, ProfileOrPool, Task};
use crate::compute::ComputeError;
use crate::config;
use crate::storage::bucket::Bucket;
use crate::storage::StorageClient;
use crate::storage::StorageError;
use bytes::Bytes;

use aws_sdk_s3::types::Object;

/// Client for the Qarnot API and chosen storage API if one is requested
pub struct QarnotClient {
    /// Client for the Qarnot API
    pub compute_client: ComputeClient,
    /// Client for the S3 service for Task data
    pub storage: Option<StorageClient>,
}

impl QarnotClient {
    /// Create a new [`QarnotClient`]
    ///
    /// # Arguments
    /// * `conf` - The configuration of the client to create
    /// # Errors
    /// * `Error::ComputeApiUnauthorized` - Credentials error
    /// * `Error::ComputeApiGeneric` - Failed to get user info from the API
    pub async fn new(conf: config::Config) -> Result<Self, Error> {
        let version = conf.version;
        let compute_url = conf.api_url;
        let api_key = conf.api_key;
        let mut storage: Option<StorageClient> = None;
        let compute_client = ComputeClient::new(compute_url, version, &api_key)?;
        if let Some(storage_url) = conf.storage_url {
            let user = compute_client.get_user_info().await?;
            info!("compute user email {:?}", user.email);

            if let Some(email) = &user.email {
                storage = Some(StorageClient::new(email, &api_key, &storage_url));
            } else {
                error!("No user email, cannot instantiate storage_client");
            }
        }
        Ok(Self {
            compute_client,
            storage,
        })
    }

    /// List buckets
    pub async fn buckets(&self) -> Result<Vec<Bucket>, Error> {
        if let Some(storage) = self.storage.as_ref() {
            let bucket_names = storage
                .buckets()
                .await
                .map_err(|_| Error::StorageApiConnect)?;
            Ok(bucket_names
                .iter()
                .filter(|b| b.name().is_some())
                .map(|b| Bucket::new(storage, b.name().unwrap_or_default()))
                .collect())
        } else {
            Err(Error::NoStorageClient)
        }
    }

    /// Get an existing bucket
    pub async fn get_bucket(&self, bucket_name: &str) -> Result<Bucket, Error> {
        if let Some(storage) = self.storage.as_ref() {
            let bucket_names = storage
                .buckets()
                .await
                .map_err(|_| Error::StorageApiConnect)?;
            let bucket = bucket_names
                .iter()
                .find(|b| b.name().map_or(false, |n| n == bucket_name));
            bucket.map_or(Err(Error::NoSuchBucket), |_| {
                Ok(Bucket::new(storage, bucket_name))
            })
        } else {
            Err(Error::NoStorageClient)
        }
    }

    /// Create a new bucket
    pub async fn create_bucket(&self, name: &str) -> Result<(), Error> {
        if let Some(storage) = self.storage.as_ref() {
            storage
                .create_bucket(name)
                .await
                .map_err(|_| Error::StorageApiConnect)
        } else {
            Err(Error::NoStorageClient)
        }
    }

    /// Delete a bucket
    pub async fn delete_bucket(&self, name: &str) -> Result<(), Error> {
        if let Some(storage) = self.storage.as_ref() {
            storage
                .delete_bucket(name)
                .await
                .map_err(|_| Error::StorageApiConnect)
        } else {
            Err(Error::NoStorageClient)
        }
    }

    /// List objects in a bucket
    pub async fn list_objects(&self, bucket: &str) -> Result<Vec<Object>, Error> {
        if let Some(storage) = self.storage.as_ref() {
            storage
                .list_objects(bucket)
                .await
                .map_err(|_| Error::StorageApiConnect)
        } else {
            Err(Error::NoStorageClient)
        }
    }

    /// Upload an object to a bucket
    pub async fn upload_object(
        &self,
        bucket: &str,
        object: crate::storage::StorageObject,
    ) -> Result<(), Error> {
        if let Some(storage) = self.storage.as_ref() {
            storage
                .upload_object(bucket, object)
                .await
                .map_err(|_| Error::StorageApiConnect)
        } else {
            Err(Error::NoStorageClient)
        }
    }

    /// Returns Bytes of an object in a bucket
    pub async fn get_object(&self, bucket: &str, key: &str) -> Result<Bytes, Error> {
        if let Some(storage) = self.storage.as_ref() {
            let stream = storage
                .get_object(bucket, key)
                .await
                .map_err(|_| Error::StorageApiConnect)?;
            let bytes = stream
                .collect()
                .await
                .map_err(|_| Error::StorageObjectDownload)?;
            Ok(bytes.into_bytes())
        } else {
            Err(Error::NoStorageClient)
        }
    }

    /// Creates a new task and returns `compute::Task` struct
    #[must_use]
    pub fn create_task(
        &self,
        name: &str,
        profile_or_pool: ProfileOrPool,
        shortname: Option<String>,
        instance_or_range: InstancesOrRange,
    ) -> Task {
        Task::new(
            &self.compute_client,
            name,
            profile_or_pool,
            shortname,
            instance_or_range,
        )
    }
}

// TODO shift compute errors and storage error to their respective clients and
// implement higher level errors with compute/storage errors as inner errors

#[derive(Debug)]
pub enum Error {
    Compute(ComputeError),
    Storage(StorageError),
    StorageApiAuth,
    StorageApiConnect,
    StorageObjectDownload,
    NoStorageClient,
    NoSuchBucket,
}

impl From<ComputeError> for Error {
    fn from(compute_error: ComputeError) -> Self {
        Self::Compute(compute_error)
    }
}
