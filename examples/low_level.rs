/// Low level SDK use example
/// Final SDK should have a higher level SDK wrapping around this low level
/// compute SDK, but this is how one could use this low-level SDK
use qarnot::client::QarnotClient;
use qarnot::compute::models::Constants;
use qarnot::compute::models::TaskCreationInput;
use qarnot::config;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let conf = config::Config::from_file("sample.conf")
        .expect("Could not create config from sample.conf file");
    let client = QarnotClient::new(conf).await.unwrap();
    // Is it up ?
    assert!(client.compute_client.get_status().await.is_ok());

    // Let's have a look at the user
    let user = client.compute_client.get_user_info().await.unwrap();
    println!("{}", serde_json::to_string(&user).unwrap());

    // What are the API versions ?
    let versions = client.compute_client.get_versions().await.unwrap();
    println!("{}", serde_json::to_string(&versions).unwrap());

    // List tasks
    let tasks = client.compute_client.get_tasks(None).await.unwrap();
    println!("{}", serde_json::to_string(&tasks).unwrap());

    // List summaries
    let summaries = client
        .compute_client
        .get_tasks_summaries(None)
        .await
        .unwrap();
    println!("{}", serde_json::to_string(&summaries).unwrap());

    // hw constraints
    let hwc = client
        .compute_client
        .get_hardware_constraints()
        .await
        .unwrap();
    println!("{}", serde_json::to_string(&hwc).unwrap());

    // profiles
    let profiles = client.compute_client.get_profiles().await.unwrap();
    println!("{}", serde_json::to_string(&profiles).unwrap());

    // profile details
    let profile = client
        .compute_client
        .get_profile_details("docker-batch")
        .await
        .unwrap();
    println!("{}", serde_json::to_string(&profile).unwrap());

    // public settings
    let settings = client.compute_client.get_settings().await.unwrap();
    println!("{}", serde_json::to_string(&settings).unwrap());

    // Post a task
    let mut task = TaskCreationInput::new(String::from("Hello Qarnot"));
    task.profile = Some(String::from("docker-batch"));
    task.instance_count = Some(1);
    let mut constants = Constants::new();
    constants.insert(
        "DOCKER_REGISTRY_LOGIN",
        &std::env::var("DOCKER_USER").expect("Missing DOCKER_USER env variable"),
    );
    constants.insert(
        "DOCKER_REGISTRY_PASSWORD",
        &std::env::var("DOCKER_PASS").expect("Missing DOCKER_PASS env variable"),
    );
    constants.insert("DOCKER_CMD", "echo HELLO QARNOT-RS");

    task.constants = Some(constants);

    let uuid = client.compute_client.post_task(task).await.unwrap();
    println!("{}", serde_json::to_string(&uuid).unwrap());

    // Post a long running task and abort it
    let mut task = TaskCreationInput::new(String::from("Hello Qarnot"));
    task.profile = Some(String::from("docker-batch"));
    task.instance_count = Some(1);
    let mut constants = Constants::new();
    constants.insert(
        "DOCKER_REGISTRY_LOGIN",
        &std::env::var("DOCKER_USER").expect("Missing DOCKER_USER env variable"),
    );
    constants.insert(
        "DOCKER_REGISTRY_PASSWORD",
        &std::env::var("DOCKER_PASS").expect("Missing DOCKER_PASS env variable"),
    );
    constants.insert("DOCKER_CMD", "sleep 3600");

    task.constants = Some(constants);

    let abort_uuid = client
        .compute_client
        .post_task(task)
        .await
        .unwrap()
        .uuid
        .unwrap();
    println!("{}", serde_json::to_string(&abort_uuid).unwrap());

    let abort_res = client.compute_client.post_abort_task(abort_uuid).await;
    match abort_res {
        Ok(_) => {
            println!("{{ \"Successfully aborted\": \"{}\"}}", abort_uuid);
        }
        Err(e) => {
            println!("Could not abort task {:?}", e);
        }
    }

    let stdout = client
        .compute_client
        .get_task_stdout(uuid.uuid.unwrap())
        .await
        .unwrap();
    println!("{}", stdout);
}
