use actix_web::{web, get, HttpResponse, Responder};

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

#[get("")]
pub async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[get("/again")]
pub async fn index2() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}

#[get("/app")]
async fn app(data: web::Data<AppState>) -> String {
  let app_name = &data.app_name; 

  format!("Hello {}!", app_name)
}




