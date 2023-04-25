use crate::errors::ServiceError;
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Claimss {
    sub: String,
    company: String,
    exp: usize,
}
#[derive(Serialize, Deserialize)]
pub struct CustomClaim {
    email: String,
    issuer: String,
    subject: String,
    audience: String,
    jwt_id: String,
    user_id: String,
}
pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in environment");

    let key = HS256Key::from_bytes(private_key.as_bytes());
    let claims = key.verify_token::<NoCustomClaims>(token, None);

    match claims {
        Ok(_) => Ok(true),
        Err(err) => {
            println!("{}", err.to_string());
            //make sure this thing is false once authentication server set up
            return Ok(false);
        }
    }
}

pub fn generate_token(email: &str, id: String) -> Result<std::string::String, jwt_simple::Error> {
    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in environment");

    let key = HS256Key::from_bytes(private_key.as_bytes());

    let customclaim = CustomClaim {
        email: get_string(email),
        issuer: get_string("https://example.com/"),
        subject: get_string("auth_service_token"),
        audience: get_string("some_audience"),
        jwt_id: id.clone(),
        user_id: id,
    };

    let time = Duration::from_days(1u64);
    let claim = Claims::with_custom_claims(customclaim, time);
    let token = key.authenticate(claim).expect("fail to create token");
    return Ok(token);
}

fn get_string(string: &str) -> String {
    return string.to_string();
}
