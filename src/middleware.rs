use std::future::{ready, Ready};

use actix_web::{error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::routes::user::Claims;

pub struct UserId(pub String);

impl FromRequest for UserId {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.headers().get("Authorization") {
            Some(token) => {
                let token = token.to_str().unwrap();
                let user_id = validate_token(token).unwrap();
                ready(Ok(UserId(user_id)))
            }
            None => ready(Err(Error::from(ErrorUnauthorized("Authorization header not found")))),
        }
    }
}

fn validate_token(token: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
    Ok(token.claims.sub.clone())
}