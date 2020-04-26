use actix_web::{get, HttpResponse, Responder};

pub mod auth;

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[get("/again")]
pub async fn index2() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}


