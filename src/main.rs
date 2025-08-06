use axum::{
    routing::get,
    Router,
};
//use std::time::Duration;
use std::collections::HashMap;
use serde_json::json;
use reqwest;


async fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("http://httpbin.org/ge").await?;
    //let headers = response.headers();
    let mut headers_json_map = HashMap::new();
    for (name, value) in response.headers().iter() {
        headers_json_map.insert(
            name.to_string(),
            value.to_str()?.to_string(), // Convert HeaderValue to String
        );
    }
    headers_json_map.insert("Response".to_string(), response.status().to_string());

    let json_output = json!(headers_json_map);

    Ok(json_output.to_string())
}

fn print_type<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>());
}
#[tokio::main]
async fn main() {
    //let data: String;
    let r = match get_ip().await {
        Ok(data) => data,
        Err(e) => e.to_string(),
    };
    println!("{r}");

    let app = Router::new().route("/healthcheck", get(|| async {r}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
