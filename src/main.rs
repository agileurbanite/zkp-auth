mod client;
mod server;

#[tokio::main]
async fn main() {
    // Start the server and client in separate tasks for demonstration
    tokio::spawn(async {
        server::run_server().await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // Give server a moment to start
    client::run_client().await;
}
