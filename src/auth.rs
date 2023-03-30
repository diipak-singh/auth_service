use crate::errors::{self, ServiceError};
use actix_service::Service;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
// use chrono::Duration;
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claimss {
    sub: String,
    company: String,
    exp: usize,
}

pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
    return Ok(true);
}

fn generate_token(key: HS256Key) -> Result<std::string::String, jwt_simple::Error> {
    let claims = Claims::create(Duration::from_hours(2));
    let token = key.authenticate(claims);
    return token;
}
