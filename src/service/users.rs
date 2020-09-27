use crate::models::users::{Claims, UpdateUser};
use bcrypt::{hash, verify};
use chrono::prelude::*;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, document::Document};
use mongodb::options::UpdateModifications;
use mongodb::results::InsertOneResult;
use mongodb::sync::Collection;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs;

// Homogenize user service errors into custom result
pub type UserServiceResult<T> = std::result::Result<T, UserServiceError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserServiceError {
    message: String,
}

// Value to return to user upon creation of user
#[derive(Deserialize, Debug, Serialize)]
pub struct MarshalledInsertOne {
    pub _id: String,
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &self.message[..];
        write!(f, "{}", s)
    }
}

// Marshall database record to JSON serialized response
fn marshall_user(d: &Document) -> Document {
    let k = doc! {
      "_id":d.get("_id").unwrap().as_object_id().unwrap().to_hex(),
      "username":d.get("username").unwrap().as_str().unwrap(),
      "email":d.get("email").unwrap().as_str().unwrap(),
      "organization":d.get("organization").unwrap().as_str().unwrap(),
      "role":d.get("role").unwrap().as_str().unwrap(),
    };
    k
}

fn get_private_key() -> String {
    let private_key_path: String = env::var("PRIVATE_KEY_PATH").unwrap();
    let private_key: String = fs::read_to_string(private_key_path).expect("Could not read file");
    private_key
}

#[derive(Clone)]
pub struct UserService {
    collection: Collection,
}

impl UserService {
    pub fn new(collection: Collection) -> UserService {
        UserService { collection }
    }

    pub fn hash_password(&self, password: &str) -> UserServiceResult<String> {
        let cost = 4;
        let hashed = hash(password, cost).map_err(|_| UserServiceError {
            message: String::from("Failure hashing password"),
        })?;
        Ok(hashed)
    }

    pub fn create(
        &self,
        username: &str,
        password: &str,
        email: &str,
        organization: &str,
        role: &str,
    ) -> UserServiceResult<MarshalledInsertOne> {
        // Check email and username unique.
        let check_username = self
            .collection
            .find_one(doc! {"username":username}, None)
            .unwrap();

        let check_email = self
            .collection
            .find_one(doc! {"email":email}, None)
            .unwrap();

        if !check_email.is_none() {
            return Err(UserServiceError {
                message: String::from("Username already in use."),
            });
        }

        if !check_username.is_none() {
            return Err(UserServiceError {
                message: String::from("Username already in use."),
            });
        }

        // Insert user
        let hashed = self.hash_password(password)?;
        let result = self
            .collection
            .insert_one(
                doc! {
                  "username": username,
                  "password":hashed,
                  "email":email,
                  "organization":organization,
                  "role":role,
                  "verified":false
                },
                None,
            )
            .map(|r: InsertOneResult| MarshalledInsertOne {
                _id: r.inserted_id.as_object_id().unwrap().to_hex(),
            })
            .map_err(|_| UserServiceError {
                message: String::from("Could not create user."),
            });
        result
    }

    pub fn get(&self, user_id: &str) -> UserServiceResult<Option<Document>> {
        let user_oid = ObjectId::with_string(user_id).map_err(|_| UserServiceError {
            message: String::from("Failure making oid object."),
        })?;
        let res = self
            .collection
            .find_one(doc! {"_id":user_oid}, None)
            // .find_one(doc! {}, None)
            .map_err(|_| UserServiceError {
                message: String::from("Failure finding user."),
            })
            .map(|r| match r {
                Some(r) => Some(marshall_user(&r)),
                None => panic!("User Not Found!"),
            });
        res
    }

    pub fn update(
        &self,
        user_id: &str,
        updates: UpdateUser,
    ) -> UserServiceResult<Option<Document>> {
        let user_oid = ObjectId::with_string(user_id).map_err(|_| UserServiceError {
            message: String::from("Failure making oid object."),
        })?;

        let mut updates_doc = Document::new();

        if updates.email.is_some() {
            updates_doc.insert("email", updates.email.unwrap());
        }

        if updates.username.is_some() {
            updates_doc.insert("username", updates.username.unwrap());
        }

        let mods = UpdateModifications::Document(doc! {"$set":updates_doc});

        let result = self
            .collection
            .find_one_and_update(doc! {"_id":user_oid}, mods, None);

        match result {
            Ok(_) => Ok(Some(doc! { "success":true})),
            _ => Err(UserServiceError {
                message: String::from("User not found."),
            }),
        }
    }

    pub fn delete(&self, user_id: &str) -> UserServiceResult<Option<Document>> {
        let user_oid = ObjectId::with_string(user_id).map_err(|_| UserServiceError {
            message: String::from("Failure making oid object."),
        })?;

        let res = self
            .collection
            .delete_one(doc! {"_id":user_oid}, None)
            .map_err(|_| UserServiceError {
                message: String::from("Failure finding user."),
            })
            .map(|_| Some(doc! {"success":true}));
        res
    }

    pub fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> UserServiceResult<Option<Document>> {
        let res = self.collection.find_one(doc! { "username":username}, None);
        if res.is_err() {
            return Err(UserServiceError {
                message: String::from("Could not query db."),
            });
        }
        let user_opt = res.unwrap();

        if user_opt.is_none() {
            return Err(UserServiceError {
                message: String::from("User not found."),
            });
        }

        let user = user_opt.unwrap();

        // println!("Getting hash");
        let hash = user.get("password").unwrap().as_str().unwrap();
        // println!("Verifying password");
        let password_ok = verify(password, hash).unwrap();

        match password_ok {
            true => {
                // println!("Wrapping Claims");
                let my_claims = Claims {
                    sub: user.get("_id").unwrap().as_object_id().unwrap().to_hex(),
                    // TODO add role
                    role: user.get("role").unwrap().as_str().unwrap().to_owned(),
                    organization: user
                        .get("organization")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_owned(),
                    exp: Utc::now().timestamp() as usize + 172800,
                };
                let h = Header::new(Algorithm::RS256);
                let k = get_private_key();
                let key = EncodingKey::from_rsa_pem(k.as_ref()).unwrap();

                let token = encode(&h, &my_claims, &key).unwrap();
                // println!("Create response document");
                let d = doc! {"token":token, "success":true};
                Ok(Some(d))
            }
            _ => Err(UserServiceError {
                message: String::from("Authentication failed."),
            }),
        }
    }
}
