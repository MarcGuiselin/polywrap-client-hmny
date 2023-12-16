use polywrap_client_hmny::{uri, Client, ClientBuilder, Uri};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .add_fs_wrap(
            uri!("hmny-wrap/test-wrap"),
            Path::new("../assets/test-wrap"),
        )
        .load()
        .await;

    let handles = vec![
        tokio::spawn(invoke_wasm(client.clone(), "a")),
        tokio::spawn(invoke_wasm(client.clone(), "b")),
    ];

    for handle in handles {
        handle.await.unwrap();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSampleMethod {
    pub arg: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SampleResult {
    pub result: String,
}

async fn invoke_wasm(client: Client, desc: &str) {
    let result: SampleResult = client
        .invoke(
            &uri!("hmny-wrap/test-wrap"),
            "sampleMethod",
            ArgsSampleMethod {
                arg: format!("{} from sample_method", desc),
            },
        )
        .await
        .unwrap();

    println!("result: {:?}", result);
}
