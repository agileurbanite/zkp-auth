use tonic::{transport::Server, Request, Response};
use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::*;

pub mod zkp_auth {
    include!("../generated/zkp_auth.rs");
}

#[derive(Debug, Default)]
pub struct MyAuthServer;

#[tonic::async_trait]
impl Auth for MyAuthServer {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, tonic::Status> {
        println!("Received register request: {:?}", request);
        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, tonic::Status> {
        println!("Received auth challenge request: {:?}", request);
        Ok(Response::new(AuthenticationChallengeResponse {
            auth_id: "123".to_string(),
            c: 123,
        }))
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, tonic::Status> {
        println!("Received verify auth request: {:?}", request);
        Ok(Response::new(AuthenticationAnswerResponse {
            session_id: "456".to_string(),
        }))
    }
}

pub async fn run_server() {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let server = MyAuthServer::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(AuthServer::new(server))
        .serve(addr)
        .await
        .unwrap();
}
