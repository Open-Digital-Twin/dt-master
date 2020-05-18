use cdrs::query_values;
use cdrs::query::*;
use cdrs::frame::TryFromRow;

// use crate::middlewares::auth::AuthorizationService;
use crate::models::user::{User, UserLogin, Claims, Register};
use crate::models::app::{Environment};
use crate::models::response::{LoginResponse, Response};
use crate::{CurrentSession};
use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
use argon2::{self, Config};
use rand::{ thread_rng, Rng };
use rand::distributions::Alphanumeric;

// use crate::routes::user::{IUserRepository, UserRepository};
// use actix_web::http::StatusCode;
// use actix_web::{post, get, web, HttpRequest, HttpResponse};
use actix_web::{post, web, HttpResponse};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[post("/login")]
async fn login(session: web::Data<Arc<CurrentSession>>, _env: web::Data<Environment>, user_login: web::Json<UserLogin>) -> HttpResponse {
  let _usr = get_user(session.clone(), user_login.email.clone());

  match _usr {
    Err(_) => {
      return HttpResponse::Ok().json(Response {
        message: format!("Invalid email {}.", user_login.email.to_string()),
        status: true
      });
    },
    Ok(user) => {   
      match authenticate(user_login.clone(), user.clone(), &_env) {
        Err(_) => {
          return HttpResponse::Ok().json(Response {
            status: false,
            message: "Invalid password informed.".to_string(),
          })
        },
        Ok(token) => {
          return HttpResponse::Ok().json(LoginResponse {
            status: true,
            token,
            message: "You have successfully logged in.".to_string(),
          });
        }
      }
    }
  }
}

fn authenticate(_login: UserLogin, user: User, _env: &Environment) -> Result<String, String> {
  if verify_hash(&_login.password, &user.password) {
    let mut _date: DateTime<Utc>;
    
    if !_login.remember_me {
      _date = Utc::now() + Duration::hours(1);
    } else {
      _date = Utc::now() + Duration::days(365);
    }
    
    let claim = Claims {
      sub: user.email,
      exp: _date.timestamp() as usize,
    };
    
    let token = encode(
      &Header::default(),
      &claim,
      &EncodingKey::from_secret(
        _env.secret_key.as_bytes()
      ),
    ).unwrap();

    return Ok(token.to_string());
  } else {
    return Err("Invalid password input".to_string());
  }
}

fn generate_hash(password: &String) -> String {
  let config = Config::default();
  let salt = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .collect::<String>();

  let hash = argon2::hash_encoded(
    &password.as_bytes(),
    &salt.as_bytes(),
    &config
  ).unwrap();

  return hash.to_string();
}

fn verify_hash(password: &String, hash: &String) -> bool {
  return argon2::verify_encoded(
    &hash.to_string(),
    &password.as_bytes()
  ).unwrap();
}

#[post("/register")]
async fn register(session: web::Data<Arc<CurrentSession>>, _env: web::Data<Environment>, user: web::Json<Register>) -> HttpResponse {
  let _usr = get_user(session.clone(), user.email.clone());

  match _usr {
    Ok(user) => HttpResponse::Ok().json(Response {
      message: format!("User {} already exists.", user.email.to_string()),
      status: false
    }),
    Err(_) => {
      session.query_with_values(
        "INSERT INTO user (email, id, name, password) VALUES (?, ?, ?, ?)",
        query_values!(
          user.email.to_string(),
          uuid::Uuid::new_v4().to_string(),
          user.name.to_string(),
          generate_hash(&user.password).to_string()
        )
      ).expect("Inserted new user");

      // TODO: Handle creation error;

      HttpResponse::Ok().json(Response {
        message: format!("Success in creating user {}.", user.email.to_string()),
        status: true
      })
    }
  }
}

fn get_user(session: web::Data<Arc<CurrentSession>>, email: String) -> Result<User, String> {
  let rows = session.query_with_values(
    "SELECT * FROM user WHERE email = ? ALLOW FILTERING",
    query_values!(email)
  )
    .expect("select user with email")
    .get_body().unwrap()
    .into_rows().unwrap();

  if !rows.is_empty() {
    let usr = match User::try_from_row(rows[0].clone()) {
      Ok(_model) => _model,
      Err(_) => return Err("Could not convert rows to User model.".to_string())
    };

    println!("User {}.", usr.email);
    return Ok(usr);
  }
  return Err("No user with selected email".to_string());
}

// #[post("/userInformations")]
// async fn user_informations(_req: HttpRequest) -> HttpResponse {
//   let _auth = _req.headers().get("Authorization");
//   let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
//   let token = _split[1].trim();
//   let _connection: Connection = Connection {};
//   let _repository: UserRepository = UserRepository {
//     connection: _connection.init(),
//   };
//   match _repository.user_informations(token) {
//     Ok(result) => HttpResponse::Ok().json(result.unwrap()),
//     Err(err) => HttpResponse::Ok().json(err),
//   }
// }

// #[get("/userInformations")]
// async fn user_informations_get(_req: HttpRequest) -> HttpResponse {
//   let _auth = _req.headers().get("Authorization");
//   let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
//   let token = _split[1].trim();
//   let _connection: Connection = Connection {};
//   let _repository: UserRepository = UserRepository {
//     connection: _connection.init(),
//   };
//   match _repository.user_informations(token) {
//     Ok(result) => HttpResponse::Ok().json(result.unwrap()),
//     Err(err) => HttpResponse::Ok().json(err),
//   }
// }

// #[post("/protectedRoute")]
// async fn protected(_: AuthorizationService) -> HttpResponse {
//   let _connection: Connection = Connection {};
//   let _repository: UserRepository = UserRepository {
//     connection: _connection.init(),
//   };
//   HttpResponse::Ok().json(_repository.protected_function())
// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(login);
  cfg.service(register);
  // cfg.service(user_informations);
  // cfg.service(user_informations_get);
  // cfg.service(protected);
}
