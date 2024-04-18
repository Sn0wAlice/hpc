// call localhost http server in post with JSON data

use reqwest::Client;
use serde_json::{json, Value};

pub async fn ask(path:String, data:Value) -> Value {
    let client = Client::new();
    let res = client.post(&format!("http://127.0.0.1:15001/{}", path))
        .json(&data)
        .send()
        .await;

    // unwrap the response
    let res = res.unwrap();

    // unwrap the body or return empty Value
    let body = res.text().await.unwrap_or("{}".to_string());
    
    // parse the body
    let body: Value = serde_json::from_str(&body).unwrap();

    // return the body
    body
}