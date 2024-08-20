#[allow(unused_imports)]
use pavex::{
    cookie::{RemovalCookie, RequestCookie, RequestCookies, ResponseCookies},
    middleware::Processing,
    response::Response,
};
use regex::Regex;
use thiserror::Error;

use crate::ctx::Ctx;

use super::AUTH_TOKEN;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing Auth Token")]
    AuthFailNoAuthTokenCookie,
    #[error("Invalid Token")]
    AuthFailTokenWrongFormat,
}

#[tracing::instrument(
    name = "mw_require_auth",
    skip(request_cookies, ctx),
    fields(
        auth_status = tracing::field::Empty,
        error = tracing::field::Empty,
    )
)]
pub async fn mw_require_auth(
    request_cookies: RequestCookies<'_>,
    ctx: &mut Ctx,
    response_cookies: &mut ResponseCookies,
) -> Result<Processing, AuthError> {
    let Some(auth_token) = request_cookies.get(AUTH_TOKEN) else {
        tracing::Span::current().record("auth_status", "fail");
        tracing::Span::current().record("error", "AuthFailNoAuthTokenCookie");
        return Err(AuthError::AuthFailNoAuthTokenCookie);
    };

    match parse_token(auth_token) {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Validate the token (expensive ops)
            ctx.set(user_id);
            tracing::Span::current().record("auth_status", "success");
            Ok(Processing::Continue)
        }
        Err(e) => {
            tracing::Span::current().record("auth_status", "fail");
            tracing::Span::current().record("error", e.to_string());
            // Err(e)
            let removal_cookie = RemovalCookie::new(AUTH_TOKEN);
            response_cookies.insert(removal_cookie);
            Ok(Processing::EarlyReturn(
                Response::unauthorized().set_typed_body("Unauthorized Basic Bitch"),
            ))
        }
    }
}

#[tracing::instrument(
    name = "parse_token",
    skip(auth_token),
    fields(
        parse_status = tracing::field::Empty,
        error = tracing::field::Empty,
    )
)]
pub fn parse_token(auth_token: RequestCookie) -> Result<(u64, String, String), AuthError> {
    let token_string = auth_token.value();

    let re = Regex::new(r"^user-(\d+)\.(.+)\.(.+)$").expect("Failed to create a regex");

    if let Some(caps) = re.captures(token_string) {
        let user_id = caps[1].parse::<u64>().map_err(|e| {
            tracing::Span::current().record("parse_status", "fail");
            tracing::Span::current().record("error", e.to_string());
            AuthError::AuthFailTokenWrongFormat
        })?;
        let exp = caps[2].to_string();
        let sign = caps[3].to_string();
        tracing::Span::current().record("parse_status", "success");
        Ok((user_id, exp, sign))
    } else {
        tracing::Span::current().record("parse_status", "fail");
        tracing::Span::current().record("error", "Bad Format");
        Err(AuthError::AuthFailTokenWrongFormat)
    }
}

// pub async fn mw_auth_error(e: &AuthError, res_cookies: &mut ResponseCookies) -> Response {
pub async fn mw_auth_error(e: &AuthError) -> Response {
    match e {
        AuthError::AuthFailNoAuthTokenCookie => {
            Response::unauthorized().set_typed_body("Unauthorized Bitch")
        }
        AuthError::AuthFailTokenWrongFormat => {
            // res_cookies.insert(RemovalCookie::new(AUTH_TOKEN));
            Response::unauthorized().set_typed_body("Unauthorized Basic Bitch")
        }
    }
}
