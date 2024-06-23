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

#[derive(Debug)]
pub enum AuthStatus {
    Success,
    LoginFail,
}

impl AuthStatus {
    pub async fn extract(
        request_head: &RequestHead,
        buffered_body: &BufferedBody,
    ) -> Result<Self, ExtractJsonBodyError> {
        // Attempt to extract the json payload from the request body
        let credentials = JsonBody::<LoginPayload>::extract(request_head, buffered_body)?;

        // If the credentials are valid json, check the authentication and return the result status
        let auth_status = if authenticate(credentials).await {
            AuthStatus::Success
        } else {
            AuthStatus::LoginFail
        };  

        Ok(auth_status)
    }
}

async fn authenticate(credentials: JsonBody<LoginPayload>) -> bool {
    // This is where you would check the credentials against a database or some other
    // authentication mechanism
    credentials.0.username == "Luca" && credentials.0.pwd == "1234"
}

pub fn invalid_credentials(_e: &ExtractJsonBodyError) -> Response {
    Response::bad_request().set_typed_body("Terrible Credentials")
}
