use pavex::{
    cookie::{RequestCookie, RequestCookies},
    middleware::Processing,
    response::Response,
};
use regex::Regex;
use thiserror::Error;

use super::AUTH_TOKEN;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Unauthorized")]
    AuthFailNoAuthTokenCookie,
    #[error("Invalid Token")]
    AuthFailTokenWrongFormat,
}

pub async fn mw_require_auth(request_cookies: RequestCookies<'_>) -> Result<Processing, AuthError> {
    let Some(_auth_token) = request_cookies.get(AUTH_TOKEN) else {
        return Err(AuthError::AuthFailNoAuthTokenCookie);
    };

    let (_user_id, _exp, _sign) = parse_token(_auth_token)?;

    // TODO: Validate the token.
    Ok(Processing::Continue)
}

pub fn parse_token(auth_token: RequestCookie) -> Result<(u64, String, String), AuthError> {
    let token_string = auth_token.value();

    let re = Regex::new(r"^user-(\d+)\.(.+)\.(.+)$").expect("Failed to create a regex");

    if let Some(caps) = re.captures(token_string) {
        let user_id = caps[1]
            .parse::<u64>()
            .map_err(|_| AuthError::AuthFailTokenWrongFormat)?;
        let exp = caps[2].to_string();
        let sign = caps[3].to_string();
        Ok((user_id, exp, sign))
    } else {
        Err(AuthError::AuthFailTokenWrongFormat)
    }
}

pub async fn mw_auth_error(e: &AuthError) -> Response {
    match e {
        AuthError::AuthFailNoAuthTokenCookie => {
            Response::unauthorized().set_typed_body("Unauthorized Bitch")
        }
        AuthError::AuthFailTokenWrongFormat => {
            Response::unauthorized().set_typed_body("Unauthorized Basic Bitch")
        }
    }
}
