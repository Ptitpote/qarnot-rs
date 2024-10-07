use log::{error, info};
use qarnot::client::QarnotClient;
use qarnot::config;
use qarnot::storage::StorageObject;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let conf = config::Config::from_file("sample.conf").unwrap();
    let client = QarnotClient::new(conf).await.unwrap();
    // Is it up ?
    assert!(client.compute_client.get_status().await.is_ok());

    let bucket_list = client.storage.as_ref().unwrap().buckets().await;
    println!("{:?}", bucket_list);

    if let Ok(list) = bucket_list {
        if !list.iter().any(|b| b.name() == Some("testbucket")) {
            let bucket = client
                .storage
                .as_ref()
                .unwrap()
                .create_bucket("testbucket")
                .await;
            println!("{:?}", bucket);
        }

        let object_list = client
            .storage
            .as_ref()
            .unwrap()
            .list_objects("testbucket")
            .await;
        println!("{:?}", object_list);

        let obj = StorageObject::new("sample.conf", "sample.conf");
        let res = client
            .storage
            .as_ref()
            .unwrap()
            .upload_object("testbucket", obj)
            .await;

        if res.is_ok() {
            info!("Successfully uploaded to bucket");
        } else {
            error!("Could not upload file to bucket");
        }

        let object_list = client
            .storage
            .as_ref()
            .unwrap()
            .list_objects("testbucket")
            .await;
        println!("{:?}", object_list);
    }
}
