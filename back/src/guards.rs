use crate::ApiError;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};



#[derive(Clone, Copy)]
pub struct AuthKey(pub [u8; 32]);

impl From<AuthKey> for String {
    fn from(key: AuthKey) -> Self {
        hex::encode(key.0)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthKey {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-auth-key") {
            Some(key_str) => {
                let mut key_bytes = [0; 32];

                if hex::decode_to_slice(key_str, &mut key_bytes).is_err() {
                    Outcome::Failure((Status::BadRequest, ApiError::AuthKeyInvalid))
                } else {
                    Outcome::Success(AuthKey(key_bytes))
                }
            }
            None => Outcome::Failure((Status::BadRequest, ApiError::AuthKeyMissing)),
        }
    }
}



#[derive(Clone)]
pub struct Email(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Email {
    type Error = ApiError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-email") {
            Some(email_str) => {
                let halves: Vec<_> = email_str.split('@').collect();

                if halves.len() != 2 {
                    Outcome::Failure((Status::BadRequest, ApiError::EmailInvalid))
                } else {
                    let mut halves_iter = halves.iter();
                    halves_iter.next().unwrap();
                    let url_half = halves_iter.next().unwrap();

                    if url_half.split('.').count() != 2 {
                        Outcome::Failure((Status::BadRequest, ApiError::EmailInvalid))
                    } else {
                        Outcome::Success(Email(String::from(email_str)))
                    }
                }
            }
            None => Outcome::Failure((Status::BadRequest, ApiError::EmailMissing)),
        }
    }
}



#[derive(Clone)]
pub struct Vault(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Vault {
    type Error = ApiError;



    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-vault") {
            Some(vault_str) => {
                if hex::decode(vault_str).is_ok() {
                    Outcome::Success(Vault(String::from(vault_str)))
                } else {
                    Outcome::Failure((Status::BadRequest, ApiError::VaultInvalid))
                }
            }
            None => Outcome::Failure((Status::BadRequest, ApiError::VaultMissing)),
        }
    }
}
