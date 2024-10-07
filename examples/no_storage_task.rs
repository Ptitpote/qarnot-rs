/// Example usage of high level task manipulation, without storage
use qarnot::client::QarnotClient;
use qarnot::compute::models::Constants;
use qarnot::config;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let conf = config::Config::from_file("sample.conf").expect("Failed to parse configuration");
    let client = QarnotClient::new(conf)
        .await
        .expect("Could not configure Qarnot Client");
    // Is it up ?
    client
        .compute_client
        .get_status()
        .await
        .expect("Could not get API status, did you configure an actual qarnot API endpoint ?");

    let mut hltask = client.create_task("hltask", "docker-batch".into(), None, 1.into());
    let mut constants = Constants::new();
    constants.insert(
        "DOCKER_REGISTRY_LOGIN",
        &std::env::var("DOCKER_USER").expect("Missing DOCKER_USER env variable"),
    );
    constants.insert(
        "DOCKER_REGISTRY_PASSWORD",
        &std::env::var("DOCKER_PASS").expect("Missing DOCKER_PASS env variable"),
    );
    constants.insert("DOCKER_CMD", "echo Super HIGH LEVEL TASK");
    hltask.constants = Some(constants);

    hltask.run().await.expect("failed to run task");
    // Was an uuid set for the task ?
    hltask
        .uuid
        .expect("Task uuid is None, task submission probably failed");
    hltask
        .wait()
        .await
        .expect("failed to wait for task completion");

    let out = hltask.stdout().await.expect("could not get task stdout");
    println!("{}", out);
}
