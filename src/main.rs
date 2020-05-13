#![allow(dead_code)]

extern crate envy;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;

use std::sync::Arc;
use std::env;

#[macro_use]
extern crate cdrs_helpers_derive;

use actix_web::http::ContentEncoding;
use actix_web::{middleware, web, App, HttpServer};

use dotenv::dotenv;


// mod middlewares;
mod models;
use crate::models::app::*;

mod routes;

pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

fn start_db_session() -> Arc<CurrentSession> {
  let node = NodeTcpConfigBuilder::new("localhost:9042", NoneAuthenticator {}).build();
  let cluster_config = ClusterTcpConfig(vec![node]);

  let _session: Arc<CurrentSession> = Arc::new(
    new_session(&cluster_config, RoundRobin::new())
      .expect("session should be created")
  );
  
  _session
}

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
  dotenv().ok();
  env_logger::init();

  HttpServer::new(move || {
    App::new()
      .data(envy::from_env::<Environment>().unwrap())
      .data(start_db_session().clone())
      .wrap(middleware::Compress::new(ContentEncoding::Br))
      .wrap(middleware::Logger::default())
      .service(web::scope("/user").configure(routes::auth::init_routes))
      // .configure(routes_config)
  })
  .bind(env::var("SERVER_ADDRESS").unwrap())?
  .run()
  .await
}
