use pavex::{
    request::{
        body::{errors::ExtractJsonBodyError, BufferedBody, JsonBody},
        RequestHead,
    },
    response::Response,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}

#[derive(Debug)]
pub enum AuthStatus {
    Success,
    LoginFail,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to extract the JSON payload")]
    ExtractJsonBody(#[from] ExtractJsonBodyError),
    #[error("Authentication Failed")]
    AuthenticationFailed,
}

impl AuthStatus {
    /// Extracts and authenticates the login payload from the request.
    #[tracing::instrument(
        name = "extract credentials",
        skip(request_head, buffered_body),
        fields(
            auth_status = tracing::field::Empty,
            error = tracing::field::Empty,
        )
    )]
    pub async fn extract(
        request_head: &RequestHead,
        buffered_body: &BufferedBody,
    ) -> Result<Self, AuthError> {
        // Attempt to extract the JSON payload from the request body.
        let credentials =
            JsonBody::<LoginPayload>::extract(request_head, buffered_body).map_err(|e| {
                tracing::Span::current().record("error", e.to_string());
                AuthError::ExtractJsonBody(e)
            })?;

        // If the credentials are valid JSON, check the authentication and return the result status.
        if authenticate(credentials).await.is_ok() {
            tracing::Span::current().record("auth_status", "success");
            Ok(AuthStatus::Success)
        } else {
            tracing::Span::current().record("auth_status", "fail");
            Ok(AuthStatus::LoginFail)
        }
    }
}

async fn authenticate(credentials: JsonBody<LoginPayload>) -> Result<(), AuthError> {
    // Check the credentials against a database or some other authentication mechanism.
    if credentials.0.username == "Luca" && credentials.0.pwd == "1234" {
        Ok(())
    } else {
        Err(AuthError::AuthenticationFailed)
    }
}

pub fn invalid_credentials(e: &AuthError) -> Response {
    match e {
        AuthError::ExtractJsonBody(_) => {
            Response::bad_request().set_typed_body("Terrible Credentials")
        }
        AuthError::AuthenticationFailed => {
            Response::unauthorized().set_typed_body("Invalid Credentials")
        }
    }
}
