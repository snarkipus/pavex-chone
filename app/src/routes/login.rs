use crate::login_payload::{AuthStatus, LoginPayload};
use pavex::request::body::JsonBody;
use pavex::response::body::Json;
use pavex::response::Response; // Add this line to import the `json` macro

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Message {
    pub result: AuthResult,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct AuthResult {
    pub success: bool,
}

pub async fn post(_body: &JsonBody<LoginPayload>, auth_status: AuthStatus) -> Response {
    if let AuthStatus::LoginFail = auth_status {
        return Response::unauthorized().set_typed_body("Invalid Credentials");
    }

    let message = Message {
        result: AuthResult { success: true },
    };

    let json = Json::new(message).expect("Failed to serialize the response body");

    Response::ok().set_typed_body(json)
}
