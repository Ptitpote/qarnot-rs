use qarnot::client::QarnotClient;
use qarnot::compute::models::Constants;
use qarnot::config;
use qarnot::storage::StorageObject;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let conf = config::Config::from_file("sample.conf")
        .expect("Could not create config from sample.conf file");
    let client = QarnotClient::new(conf).await.unwrap();
    // Is it up ?
    assert!(client.compute_client.get_status().await.is_ok());

    // Clean previous run input/output
    let _ = client
        .storage
        .as_ref()
        .unwrap()
        .delete_object("testinput", "examples/sample_rust.rs")
        .await;
    let _ = client
        .storage
        .as_ref()
        .unwrap()
        .delete_object("testinput", "examples")
        .await;

    let bucket_list = client.buckets().await;
    if let Ok(list) = bucket_list {
        if !list.iter().any(|b| b.name.as_str() == "testinput") {
            let bucket = client.create_bucket("testinput").await;
            println!("{:?}", bucket);
        }
        if !list.iter().any(|b| b.name.as_str() == "testoutput") {
            let bucket = client.create_bucket("testoutput").await;
            println!("{:?}", bucket);
        }
    }

    let bucket = client.get_bucket("testinput").await.unwrap();

    let obj = StorageObject::new("examples/sample_rust.rs", "sample_rust.rs");
    bucket
        .upload_object(obj)
        .await
        .expect("Could not upload object");

    let mut task = client.create_task("Rust compile", "docker-batch".into(), None, 1.into());

    let mut constants = Constants::new();
    constants.insert(
        "DOCKER_REGISTRY_LOGIN",
        &std::env::var("DOCKER_USER").expect("Missing DOCKER_USER env variable"),
    );
    constants.insert(
        "DOCKER_REGISTRY_PASSWORD",
        &std::env::var("DOCKER_PASS").expect("Missing DOCKER_PASS env variable"),
    );
    constants.insert("DOCKER_REPO", "rust");
    constants.insert("DOCKER_TAG", "1.80-alpine3.20");
    constants.insert("DOCKER_CMD", "rustc --out-dir /job /job/sample_rust.rs");

    task.constants = Some(constants);
    task.resouce_buckets = Some(vec![String::from("testinput")]);
    task.result_bucket = Some(String::from("testoutput"));

    task.run().await.expect("Failed to run task");

    task.uuid
        .expect("No task uuid, task submission probably failed");

    let _ = task.wait().await;

    let stdout = task.stdout().await.expect("Failed to get task stdout");
    println!("{}", stdout);

    let result_obj = StorageObject::new("sample_rust", "sample_rust");
    let result_bucket = client.get_bucket("testoutput").await.unwrap();
    result_bucket.get_object(result_obj).await.unwrap();
}
