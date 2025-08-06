use axum::{
    routing::get,
    Router,
};
use std::time::Duration;
use reqwest;


async fn get_ip() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = match client.get("https://httpbin.org/ip")
        .timeout(Duration::from_secs(10))  
        .send()
        .await {
            Ok(res) => {
                if res.status().is_client_error() {
                    Err(format!("Server returned a client error: {}", res.status()).into())
                } else if res.status().is_server_error() {
                    return Err(format!("Server returned a error, check your request data: {}", res.status()).into());
                }               
                else {
                    Ok(res)
                }
            }
            Err(err) => {
                if err.is_connect() {
                    return Err("Failed to connect to the server. Please make sure the server is running.".into());
                }
                else if err.is_timeout() {
                    return Err(format!("Request timed out: {}", err).into());
                } else {
                    return Err(Box::new(err));
                }
            }
        };
    println!("{:?}", response);
    // Handle the response and extract JSON
    let result_json: serde_json::Value = match response {
        Ok(res) => match res.json().await {
            Ok(json) => json,
            Err(err) => return Err(Box::new(err)),
        },
        Err(err) => return Err(err),
    };
 
    Ok(result_json)
}

fn print_type<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>());
}
#[tokio::main]
async fn main() {
    //let data: String;
    match get_ip().await {
        Ok(data) => println!("Success: {}", data),
        Err(e) => eprintln!("Error: {}", e),
    }

    //let mut app;
    //
    

    let app = Router::new().route("/healthcheck", get(|| async { "200 OK" }));
    print_type(&app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
