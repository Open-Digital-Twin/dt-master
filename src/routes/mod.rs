pub mod auth;
// pub mod user;

// pub use auth::init_routes;
// pub use user::{IUserRepository, UserRepository};

// use crate::db::db::{Connection, IConnection};
// use crate::middlewares::auth::AuthorizationService;
// use crate::models::user::{Login, Register};
// use crate::routes::user::{IUserRepository, UserRepository};
// use actix_web::http::StatusCode;
// use actix_web::{web, get, HttpResponse};


// #[post("/login")]
// async fn login(user: web::Json<Login>) -> HttpResponse {
//   let _connection: Connection = Connection {};
//   let _repository: UserRepository = UserRepository {
//     connection: _connection.init(),
//   };
//   let proc = _repository.login(user.into_inner());

//   match proc {
//     Ok(_) => HttpResponse::Ok().json(proc.unwrap()),
//     Err(_) => HttpResponse::Ok()
//       .status(StatusCode::from_u16(401).unwrap())
//       .json(proc.unwrap_err()),
//   }
// }

