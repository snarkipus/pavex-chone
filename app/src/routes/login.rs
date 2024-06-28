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

pub async fn post(
    _body: &JsonBody<LoginPayload>,
    auth_status: AuthStatus,
    response_cookies: &mut ResponseCookies,
) -> Response {
    match auth_status {
        AuthStatus::Success => {
            // Set a cookie to indicate that the user is authenticated.
            let cookie = ResponseCookie::new(AUTH_TOKEN, "user-1.exp.sign");
            response_cookies.insert(cookie);

            let message = Message {
                result: AuthResult { success: true },
            };

            let json = Json::new(message).expect("Failed to serialize the response body");
            Response::ok().set_typed_body(json)
        }
        AuthStatus::LoginFail => Response::unauthorized().set_typed_body("Invalid Credentials"),
    }
}
