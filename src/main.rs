use actix_web::{App, HttpServer};

mod routes;
use routes::{index, index2};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(index)
      .service(index2)
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}
