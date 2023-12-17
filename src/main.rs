use polywrap_client_hmny::{uri, Client, ClientBuilder, ClosureWrap, Uri};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .add_file(uri!("hmny-wrap/test-wrap"), Path::new("./assets/test-wrap"))
        .add_closure(
            uri!("hmny-core/test-wrap"),
            ClosureWrap::new().add_method("sampleMethod", |args: &ArgsSampleMethod| {
                Ok(SampleResult {
                    result: format!("{} from hmny-core/test-wrap", args.arg),
                })
            }),
        )
        .load()
        .await
        .expect("failed to load wraps");

    let handles = vec![
        tokio::spawn(invoke_wasm(
            client.clone(),
            uri!("hmny-wrap/test-wrap"),
            "a",
        )),
        tokio::spawn(invoke_wasm(
            client.clone(),
            uri!("hmny-core/test-wrap"),
            "b",
        )),
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

async fn invoke_wasm(client: Client, uri: Uri, desc: &str) {
    let result: SampleResult = client
        .invoke(
            &uri,
            "sampleMethod",
            ArgsSampleMethod {
                arg: format!("{} from sample_method", desc),
            },
        )
        .await
        .unwrap();

    println!("result: {:?}", result);
}
