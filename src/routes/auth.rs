use cdrs::query_values;
use cdrs::query::*;
use cdrs::frame::TryFromRow;

// use crate::middlewares::auth::AuthorizationService;
use crate::models::user::{User, Login, Register};
use crate::models::response::{LoginResponse, Response};
// use crate::models::app::{AppState};
use crate::{CurrentSession};
use std::sync::Arc;

// use crate::routes::user::{IUserRepository, UserRepository};
// use actix_web::http::StatusCode;
// use actix_web::{post, get, web, HttpRequest, HttpResponse};
use actix_web::{post, web, HttpResponse};


#[post("/register")]
async fn register(session: web::Data<Arc<CurrentSession>>, user: web::Json<Register>) -> HttpResponse {
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
          user.password.to_string()
        )
      ).expect("Inserted new user");

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
