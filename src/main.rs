use actix_web::http::ContentEncoding;
use actix_web::{middleware, web, App, HttpServer};

// mod config;
mod db;
// mod middlewares;
mod models;
// mod routes;

// fn insert_struct(session: &CurrentSession) {
//     let row = Twin {
//       id: 300000i64,
//       name: "Maike".to_string()
//     };

//     let insert_struct_cql = "INSERT INTO test_ks.twin (id, name) VALUES (?, ?)";

//     session
//       .query_with_values(insert_struct_cql, row.into_query())
//       .expect("insert");
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let session = start_db_session();
  init_db(&session);

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Compress::new(ContentEncoding::Br))
      .wrap(middleware::Logger::default())
      .service(web::scope("/user").configure(routes::init_routes))
      // .configure(routes_config)
  })
  .bind("0.0.0.0:3000")?
  .run()
  .await
}
