use actix_web::{web, get, HttpResponse, Responder, Result};

// pub mod auth;
use super::AppState as AppState;

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::scope("/")
        .data(AppState {
          app_name: String::from("Test")
        })
        .service(index)
        .service(index2)
        .service(app)
    );

    /*
    cfg.service(
      web::scope("/auth")
    )
    */
}

#[get("users/{id}/{name}")]
async fn index(
  info: web::Path<(u32, String)>
) -> Result<String> {
  Ok(format!("User #{}: \"{}\".", info.0, info.1))
}

#[get("/again")]
async fn index2() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}

#[get("/app")]
async fn app(data: web::Data<AppState>) -> String {
  let app_name = &data.app_name; 

  format!("Hello {}!", app_name)
}




