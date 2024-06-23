use pavex::{
    request::{
        body::{errors::ExtractJsonBodyError, BufferedBody, JsonBody},
        RequestHead,
    },
    response::Response,
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}

pub enum AuthStatus {
    Invalid,
    Valid(LoginPayload),
}

impl AuthStatus {
    pub fn extract(
        request_head: &RequestHead,
        buffered_body: &BufferedBody,
    ) -> Result<Self, ExtractJsonBodyError> {
        let Ok(credentials) = JsonBody::<LoginPayload>::extract(request_head, buffered_body) else {
            return Ok(AuthStatus::Invalid);
        };

        let (name, pwd) = (&credentials.0.username, &credentials.0.pwd);
        if name == "Luca" && pwd == "1234" {
            Ok(AuthStatus::Valid(LoginPayload {
                username: name.clone(),
                pwd: pwd.clone(),
            }))
        } else {
            Ok(AuthStatus::Invalid)
        }
    }
}

pub fn invalid_credentials(_e: &ExtractJsonBodyError) -> Response {
    Response::bad_request().set_typed_body("It never gets here anyways")
}
