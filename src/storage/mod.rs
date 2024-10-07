use aws_sdk_s3::config::{http::HttpResponse, BehaviorVersion, Credentials, Region};
use aws_sdk_s3::operation::{
    create_bucket::CreateBucketError, delete_bucket::DeleteBucketError,
    delete_object::DeleteObjectError, get_object::GetObjectError, list_buckets::ListBucketsError,
    list_objects_v2::ListObjectsV2Error,
};
use aws_sdk_s3::primitives::SdkBody;
use aws_sdk_s3::types::{Bucket, Object};
use aws_sdk_s3::Config;
use aws_smithy_runtime_api::{client::result::SdkError, http::Response};
use aws_smithy_types::byte_stream::ByteStream;

pub mod bucket;

//TODO Make a StorageClient TRAIT that can be implemented however users like,
//then make QarnotClient<S> where S impl StorageClient, allowing multiple implems
//of StorageClient that can use other implems of S3 libs (the horrendous aws official
//experimental rust SDK for example).

#[derive(Debug)]
pub enum StorageError {
    LocalFileDoesNotExist,
    NoSuchBucket,
    UploadError,
    Generic,
}

pub struct StorageObject {
    pub local_path: String,
    pub key: String,
}

impl StorageObject {
    pub fn new(local_path: &str, key: &str) -> Self {
        Self {
            local_path: String::from(local_path),
            key: String::from(key),
        }
    }
}

/// Lower level wrapper around S3 client
/// Preferably use QarnotClient methods around buckets that wrap everything
/// in higher level Bucket structurs (cf: `bucket` module)
pub struct StorageClient {
    s3_client: aws_sdk_s3::Client,
}

impl StorageClient {
    pub fn new(access_key: &str, secret_key: &str, storage_url: &str) -> Self {
        let creds = Credentials::new(access_key, secret_key, None, None, "my_creds");
        let config = Config::builder()
            .endpoint_url(storage_url)
            .endpoint_resolver(aws_sdk_s3::config::endpoint::DefaultResolver::new())
            .credentials_provider(creds)
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new("eu-west-1"))
            .build();
        let s3_client = aws_sdk_s3::Client::from_conf(config);
        Self { s3_client }
    }

    /// Provide your own s3 client
    pub const fn new_custom_client(s3_client: aws_sdk_s3::Client) -> Self {
        Self { s3_client }
    }

    pub async fn create_bucket(
        &self,
        name: &str,
    ) -> Result<(), SdkError<CreateBucketError, HttpResponse>> {
        let res = self.s3_client.create_bucket().bucket(name).send().await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// List user buckets
    pub async fn buckets(&self) -> Result<Vec<Bucket>, SdkError<ListBucketsError, HttpResponse>> {
        let res = self.s3_client.list_buckets().send().await;
        match res {
            Ok(bucket_list) => Ok(bucket_list.buckets.unwrap_or_default()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_bucket(
        &self,
        name: &str,
    ) -> Result<(), SdkError<DeleteBucketError, HttpResponse>> {
        let res = self.s3_client.delete_bucket().bucket(name).send().await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn list_objects(
        &self,
        bucket: &str,
    ) -> Result<Vec<Object>, SdkError<ListObjectsV2Error, HttpResponse>> {
        let res = self.s3_client.list_objects_v2().bucket(bucket).send().await;
        match res {
            Ok(response) => Ok(response.contents().to_vec()),
            Err(e) => Err(e),
        }
    }

    pub async fn upload_object(
        &self,
        bucket: &str,
        object: StorageObject,
    ) -> Result<(), StorageError> {
        let stream = ByteStream::from_path(&object.local_path).await;
        if let Ok(stream) = stream {
            self.s3_client
                .put_object()
                .key(object.key)
                .bucket(bucket)
                .body(stream)
                .send()
                .await
                .map_err(|_| StorageError::LocalFileDoesNotExist)?;
            Ok(())
        } else {
            error!("{:?}", stream);
            Err(StorageError::LocalFileDoesNotExist)
        }
    }

    pub async fn delete_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<(), SdkError<DeleteObjectError, HttpResponse>> {
        let res = self
            .s3_client
            .delete_object()
            .key(key)
            .bucket(bucket)
            .send()
            .await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<ByteStream, SdkError<GetObjectError, Response<SdkBody>>> {
        let res = self
            .s3_client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await;
        match res {
            Ok(res) => Ok(res.body),
            Err(e) => Err(e),
        }
    }
}
