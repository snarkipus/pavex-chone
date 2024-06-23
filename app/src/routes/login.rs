use crate::login_payload::{AuthStatus, LoginPayload};
use pavex::request::body::JsonBody;
use pavex::response::Response;

pub async fn post(body: &JsonBody<LoginPayload>, auth_status: AuthStatus) -> Response {
    match auth_status {
      AuthStatus::Invalid => {
        Response::unauthorized().set_typed_body("Invalid Credentials")
      }
      _ => {
        let (name, _pwd) = (&body.0.username, &body.0.pwd);
        Response::ok().set_typed_body(format!("Hello, {name}!"))
      }
    }
}
