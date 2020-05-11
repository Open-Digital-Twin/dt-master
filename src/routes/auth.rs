use cdrs::query_values;
use cdrs::query::*;
use cdrs::frame::TryFromRow;

// use crate::middlewares::auth::AuthorizationService;
use crate::models::user::{User, Login};
use crate::models::response::{LoginResponse};
// use crate::models::app::{AppState};
use crate::{CurrentSession};
use std::sync::Arc;

// use crate::routes::user::{IUserRepository, UserRepository};
// use actix_web::http::StatusCode;
// use actix_web::{post, get, web, HttpRequest, HttpResponse};
use actix_web::{post, web, HttpResponse};

#[post("/login")]
async fn login(session: web::Data<Arc<CurrentSession>>, login: web::Json<Login>) -> HttpResponse {
  
  let rows = session.query_with_values(
    "SELECT * FROM user WHERE email = ?",
    query_values!(login.email.to_string())
  )
    .expect("select user with email")
    .get_body().unwrap()
    .into_rows().unwrap();

  for row in rows {
    let my_row: User = User::try_from_row(row).unwrap();
    println!("struct got: {:?}", my_row);
  }

  // let _repository: UserRepository = UserRepository {
  //   connection: _connection.init(),
  // };
  // let proc = _repository.login(user.into_inner());
  HttpResponse::Ok().json(LoginResponse {
    status: true,
    token: login.email.to_string(),
    message: "You have successfully logged in.".to_string(),
  })

  // match proc {
  //   Ok(_) => HttpResponse::Ok().body({}),//json(proc.unwrap()),
  //   Err(_) => HttpResponse::Ok()
  //     .status(StatusCode::from_u16(401).unwrap())
  //     .json(proc.unwrap_err()),
  // }
}

// #[post("/register")]
// async fn register(user: web::Json<Register>) -> HttpResponse {
//   let _connection: Connection = Connection {};
//   let _repository: UserRepository = UserRepository {
//     connection: _connection.init(),
//   };
//   HttpResponse::Ok().json(_repository.register(user.into_inner()))
// }

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
  // cfg.service(register);
  // cfg.service(user_informations);
  // cfg.service(user_informations_get);
  // cfg.service(protected);
}
