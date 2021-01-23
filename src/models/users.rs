extern crate regex;

use actix_web::error::ErrorBadRequest;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use regex::Regex;

use std::env;
use std::fs;

fn get_public_key() -> String {
    let public_key_path: String = env::var("PUBLIC_KEY_PATH").unwrap();
    let public_key: String = fs::read_to_string(public_key_path).expect("Could not read file");
    return public_key
}

lazy_static! {
    static ref RE_ROLE_ENUM: Regex = Regex::new(r"(User|Admin)").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub organization: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct User {
    #[validate(length(min = 3))]
    pub username: String,
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub organization: String,
    #[validate(regex = "RE_ROLE_ENUM")]
    pub role: String,
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 3))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct AuthorizedUser {
    pub token: String,
    pub sub: String,
    pub role: String,
    pub organization: String,
}

impl FromRequest for AuthorizedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        match req.headers().get("Authorization") {
            Some(val) => match val.to_str() {
                Ok(v) => {
                    //
                    // let t = v.into();
                    let my_slice: Vec<&str> = v.split(" ").collect();
                    let k = get_public_key();
                    let key = DecodingKey::from_rsa_pem(k.as_ref()).unwrap();
                    // let token = encode(&Header::default(), &my_claims, &key).unwrap();
                    // TODO: check that my_slice has length of 2.
                    print!("{}", &v);

                    if my_slice.len() != 2 {
                        return err(ErrorBadRequest("Bad Headers"));
                    }

                    let token_claims_res =
                        decode::<Claims>(my_slice[1], &key, &Validation::new(Algorithm::RS256));

                    if token_claims_res.is_err() {
                        // dbg!("{:?}", token_claims_res);
                        return err(ErrorBadRequest("Not Authorized"));
                    }

                    let claims = token_claims_res.unwrap().claims;
                    let authorized_user = AuthorizedUser {
                        token: v.into(),
                        sub: claims.sub,
                        role: claims.role,
                        organization: claims.organization,
                    };
                    ok(authorized_user)
                }
                Err(e) => err(ErrorBadRequest(e)),
            },
            None => err(ErrorBadRequest("Not Authorized")),
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct AuthenticateUser {
    #[validate(length(min = 3))]
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Info {
    pub user_id: String,
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct UserResponseData {
    _id: String,
}
