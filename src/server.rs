use std::collections::HashMap;
use std::sync::RwLock;
use tonic::{transport::Server, Request, Response};
use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::*;

pub mod zkp_auth {
    include!("../generated/zkp_auth.rs");
}

#[derive(Debug, Default)]
pub struct MyAuthServer {
    users: RwLock<HashMap<String, (i64, i64)>>,
}

#[tonic::async_trait]
impl Auth for MyAuthServer {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, tonic::Status> {
        let RegisterRequest { user, y1, y2 } = request.into_inner();
        let mut users = self.users.write().unwrap();

        if users.contains_key(&user) {
            return Err(tonic::Status::already_exists("User already registered."));
        }

        users.insert(user, (y1, y2));

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, tonic::Status> {
        let user = request.into_inner().user;
        let users = self.users.read().unwrap();

        if users.contains_key(&user) {
            // Here, a real-world scenario would create a challenge based on the user's data.
            // But for simplicity, we're returning fixed values.
            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id: format!("ChallengeFor{}", user),
                c: 123,
            }))
        } else {
            Err(tonic::Status::not_found("User not found."))
        }
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, tonic::Status> {
        let auth_id = request.into_inner().auth_id;

        // Here, we'd verify the answer given a challenge. For now, just checking the existence of the auth_id.
        if auth_id.starts_with("ChallengeFor") {
            Ok(Response::new(AuthenticationAnswerResponse {
                session_id: format!("SessionFor{}", &auth_id["ChallengeFor".len()..]),
            }))
        } else {
            Err(tonic::Status::invalid_argument("Invalid auth_id."))
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Code;

    #[tokio::test]
    async fn test_registration() {
        let server = MyAuthServer::default();

        // Successful registration
        let response = server
            .register(Request::new(RegisterRequest {
                user: "Alice".to_string(),
                y1: 1,
                y2: 2,
            }))
            .await;
        assert!(response.is_ok());

        // Duplicate registration
        let response = server
            .register(Request::new(RegisterRequest {
                user: "Alice".to_string(),
                y1: 3,
                y2: 4,
            }))
            .await;
        assert_eq!(response.err().unwrap().code(), Code::AlreadyExists);
    }

    #[tokio::test]
    async fn test_authentication_challenge() {
        let server = MyAuthServer::default();

        // Request challenge for non-existent user
        let response = server
            .create_authentication_challenge(Request::new(AuthenticationChallengeRequest {
                user: "Alice".to_string(),
                r1: 1,
                r2: 2,
            }))
            .await;
        assert_eq!(response.err().unwrap().code(), Code::NotFound);

        // Register the user first
        server
            .register(Request::new(RegisterRequest {
                user: "Alice".to_string(),
                y1: 1,
                y2: 2,
            }))
            .await
            .unwrap();

        // Now request the challenge
        let response = server
            .create_authentication_challenge(Request::new(AuthenticationChallengeRequest {
                user: "Alice".to_string(),
                r1: 1,
                r2: 2,
            }))
            .await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().into_inner().auth_id, "ChallengeForAlice");
    }

    #[tokio::test]
    async fn test_verify_authentication() {
        let server = MyAuthServer::default();

        // Verification with invalid auth_id
        let response = server
            .verify_authentication(Request::new(AuthenticationAnswerRequest {
                auth_id: "InvalidChallenge".to_string(),
                s: 1,
            }))
            .await;
        assert_eq!(response.err().unwrap().code(), Code::InvalidArgument);

        // Verification with valid format of auth_id (though no previous challenge was made)
        let response = server
            .verify_authentication(Request::new(AuthenticationAnswerRequest {
                auth_id: "ChallengeForAlice".to_string(),
                s: 1,
            }))
            .await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().into_inner().session_id, "SessionForAlice");
    }
}
