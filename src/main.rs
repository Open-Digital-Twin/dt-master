use actix_web::{App, HttpServer};

mod routes;
use routes::routes_config;

struct AppState {
  app_name: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .data(AppState {
        app_name: String::from("Digital Twin")
      })
      .configure(routes_config)
  })
  .bind("0.0.0.0:3000")?
  .run()
  .await
}
