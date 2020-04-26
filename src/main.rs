#[macro_use]
extern crate bson;

use actix_web::http::ContentEncoding;
use actix_web::{middleware, web, App, HttpServer};

mod routes;
use routes::routes_config;

mod config;
mod db;
mod middlewares;
mod models;
mod repositories;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Compress::new(ContentEncoding::Br))
      .wrap(middleware::Logger::default())
      .service(web::scope("/user").configure(repositories::user_repository::init_routes))
      .configure(routes_config)
  })
  .bind("0.0.0.0:3000")?
  .run()
  .await
}
