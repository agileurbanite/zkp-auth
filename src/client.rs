use tonic::{transport::Channel, Request};
use zkp_auth::auth_client::AuthClient;
use zkp_auth::*;

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth"); // The string specified here must match the proto package name
}

pub async fn run_client() {
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await
        .unwrap();

    let mut client = AuthClient::new(channel);

    let request = Request::new(RegisterRequest {
        user: "Alice".to_string(),
        y1: 1,
        y2: 2,
    });

    let response = client.register(request).await.unwrap();
    println!("Received response: {:?}", response);
}
