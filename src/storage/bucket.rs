use crate::storage::{StorageClient, StorageObject};
use aws_sdk_s3::types::Object;
use aws_smithy_runtime_api::client::result::SdkError;

pub struct Bucket<'a> {
    client: &'a StorageClient,
    pub name: String,
}

impl<'a> Bucket<'a> {
    pub fn new(client: &'a StorageClient, name: &str) -> Self {
        Self {
            client,
            name: name.to_owned(),
        }
    }

    pub async fn list_objects(&self) -> Result<Vec<Object>, Error> {
        self.client
            .list_objects(&self.name)
            .await
            .map_err(Error::from)
    }

    //TODO Proper error handling
    pub async fn upload_object(&self, object: StorageObject) -> Result<(), Error> {
        self.client
            .upload_object(&self.name, object)
            .await
            .map_err(|_| Error::Unknown)
    }

    /// Download object key to object path
    pub async fn get_object(&self, object: StorageObject) -> Result<(), Error> {
        let obj = self
            .client
            .get_object(&self.name, &object.key)
            .await
            .map_err(Error::from)?;
        let bytes = obj.collect().await;
        match bytes {
            Ok(bytes) => {
                std::fs::write(object.local_path, bytes.into_bytes()).map_err(|_| Error::WriteFile)
            }
            Err(_) => Err(Error::CollectObjectBytes),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    CollectObjectBytes,
    RequestConstructionFailed,
    InvalidResponse,
    NoResponse,
    NoSuchBucket,
    NoSuchObject,
    ServiceError,
    Timeout,
    WriteFile,
    Unknown,
}

impl<E, R> From<SdkError<E, R>> for Error {
    fn from(e: SdkError<E, R>) -> Self {
        match e {
            SdkError::ConstructionFailure(_) => Self::RequestConstructionFailed,
            SdkError::DispatchFailure(_) => Self::NoResponse,
            SdkError::TimeoutError(_) => Self::Timeout,
            SdkError::ResponseError(_) => Self::InvalidResponse,
            SdkError::ServiceError(_) => Self::ServiceError,
            _ => Self::Unknown,
        }
    }
}
