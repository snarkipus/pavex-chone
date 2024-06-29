use crate::credentials::{AuthStatus, LoginPayload};
use pavex::cookie::{ResponseCookie, ResponseCookies};
use pavex::request::body::JsonBody;
use pavex::response::body::Json;
use pavex::response::Response;

use super::web::AUTH_TOKEN;

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Message {
    pub result: AuthResult,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AuthResult {
    pub success: bool,
}

#[tracing::instrument(
    name = "login",
    skip(_body, response_cookies),
    fields(
        auth_status = ?auth_status,
        success = tracing::field::Empty,
        error = tracing::field::Empty
    )
)]
pub async fn post(
    _body: &JsonBody<LoginPayload>,
    auth_status: AuthStatus,
    response_cookies: &mut ResponseCookies,
) -> Response {
    let result = match auth_status {
        AuthStatus::Success => {
            // Set a cookie to indicate that the user is authenticated.
            let cookie = ResponseCookie::new(AUTH_TOKEN, "user-1.exp.sign");
            response_cookies.insert(cookie);

            tracing::Span::current().record("success", true);

            let message = Message {
                result: AuthResult { success: true },
            };

            let json = Json::new(message).expect("Failed to serialize the response body");
            Response::ok().set_typed_body(json)
        }
        AuthStatus::LoginFail => {
            tracing::Span::current().record("error", "Invalid Credentials");
            Response::unauthorized().set_typed_body("Invalid Credentials")
        }
    };

    result
}
