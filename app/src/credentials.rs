use pavex::{
    request::{
        body::{errors::ExtractJsonBodyError, BufferedBody, JsonBody},
        RequestHead,
    },
    response::Response,
};
use thiserror::Error;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}

#[derive(Debug)]
pub enum AuthStatus {
    Success,
    LoginFail(AuthError),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("JSON Payload is busted AF")]
    ExtractJsonBody(#[from] ExtractJsonBodyError),
    #[error("Authentication Failed")]
    AuthenticationFailed,
}

impl AuthStatus {
    pub async fn extract(
        request_head: &RequestHead,
        buffered_body: &BufferedBody,
    ) -> Result<Self, AuthError> {
        // Attempt to extract the json payload from the request body
        let credentials = JsonBody::<LoginPayload>::extract(request_head, buffered_body)
            .map_err(AuthError::ExtractJsonBody)?;

        // If the credentials are valid json, check the authentication and return the result status
        if authenticate(credentials).await.is_ok() {
            Ok(AuthStatus::Success)
        } else {
            Ok(AuthStatus::LoginFail(AuthError::AuthenticationFailed))
        }
    }
}

async fn authenticate(credentials: JsonBody<LoginPayload>) -> Result<(), AuthError> {
    // This is where you would check the credentials against a database or some other
    // authentication mechanism
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
