use log::*;
use tide::{Body, Error, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};

use crate::db::model::ProvideError;

/// The signing key used to mint auth tokens
pub const SECRET_KEY: &str = "this-is-the-most-secret-key-ever-secreted";

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub exp: i64,
}

/// Retrieve the authorization header from a Request
fn get_auth_header<T>(req: &Request<T>) -> Option<&str> {
    // TODO: It is possible the user will provide multiple auth headers, we should try all of them
    req.header("Authorization").map(|h| h.last().as_str())
}

/// Extract the JWT token from a header string
fn parse_token(header: &str) -> String {
    header.splitn(2, ' ').nth(1).unwrap_or_default().to_owned()
}

/// Authorize a JWT returning the user_id
fn authorize_token(token: &str) -> jsonwebtoken::errors::Result<i32> {
    let data = jsonwebtoken::decode::<TokenClaims>(
        token,
        SECRET_KEY.as_ref(),
        &jsonwebtoken::Validation::default(),
    )?;

    Ok(data.claims.sub)
}

/// Validate an auth token if one is present in the request
///
/// This is useful for routes where auth is optional (e.g. /api/get/articles
///
/// 1. No authorization header present -> None
/// 2. Invalid authorization header -> Some(Error)
/// 3. Valid authorization header -> Some(Ok)
pub fn optionally_auth<T>(req: &Request<T>) -> Option<Result<(i32, String), Response>> {
    if req.header("Authorization").is_some() {
        Some(extract_and_validate_token(req))
    } else {
        None
    }
}

/// Validates an auth token from a Request, returning the user ID and token if successful
pub fn extract_and_validate_token<T>(req: &Request<T>) -> Result<(i32, String), Response> {
    debug!("Checking for auth header");
    let auth_header = get_auth_header(&req)
        .ok_or_else(|| err_response(StatusCode::BadRequest, "Missing Authorization header"))?;

    debug!("Extracting token from auth header");
    let token = parse_token(auth_header);

    debug!("Authorizing token");
    let user_id =
        authorize_token(&token)
            .map_err(|e| err_response(StatusCode::Forbidden, e.to_string()))?;

    debug!("Token is valid and belongs to user {}", user_id);

    Ok((user_id, token))
}

/// Converts a serializable payload into a JSON response
///
/// If the body cannot be serialized an Err(Response) will be returned with the serialization error
pub fn to_json_response<B: Serialize>(
    body: &B,
    status: StatusCode,
) -> Response {
    let mut resp = Response::new(status);
    match Body::from_json(body) {
        Ok(json) => {
            resp.set_body(json);
            resp
        }
        Err(e) => {
            let error_msg = format!("Failed to serialize response -- {}", e);
            warn!("{}", error_msg);
            resp.set_status(StatusCode::InternalServerError);
            resp.set_body(error_msg);
            resp
        }
    }
}

/// Create an error response payload with the procided Status and message
pub fn err_response(status: StatusCode, message: impl AsRef<str>) -> Response {
    let mut resp = Response::new(status);
    #[derive(Serialize)]
    struct ErrorResponseBody<'a> {
        errors: Inner<'a>
    }
    #[derive(Serialize)]
    struct Inner<'a> {
        body: &'a [&'a str]
    }

    let payload = ErrorResponseBody { errors: Inner { body: &[message.as_ref()] } };
    let body = Body::from_json(&payload).expect("Failed to serialize");
    resp.set_body(body);
    resp

}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Missing Authorization header")]
    MissingAuthHeader
}

impl From<ProvideError> for Response {
    /// Convert a ProvideError into a [tide::Response]
    ///
    /// This allows the usage of
    fn from(e: ProvideError) -> Response {
        let mut resp = Response::new(500);
        match e {
            ProvideError::NotFound => resp.set_status(StatusCode::NotFound),
            ProvideError::Provider(e) => {
                resp.set_status(StatusCode::InternalServerError);
                resp.set_body(e.to_string());
            }
            ProvideError::UniqueViolation(details) => {
                resp.set_status(StatusCode::Conflict);
                resp.set_body(details)
            }
            ProvideError::ModelViolation(details) => {
                resp.set_status(StatusCode::BadRequest);
                resp.set_body(details)
            }
        };
        resp
    }
}
