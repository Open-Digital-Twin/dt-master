#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;

use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

use lazy_static::lazy_static;

type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

fn start_db_session() -> CurrentSession {
  let node = NodeTcpConfigBuilder::new("localhost:9042", NoneAuthenticator {}).build();
  let cluster_config = ClusterTcpConfig(vec![node]);

  let session: CurrentSession = new_session(&cluster_config, RoundRobin::new())
    .expect("session should be created");
  
  session
}

// fn init_db(session: &CurrentSession) {
//   let mut file = File::open("db.cql")?;
//   let mut contents = String::new();
//   file.read_to_string(&mut contents)?;
//   session.query(contents).expect("DB init error");
// }

lazy_static! {
  static ref session: CurrentSession = start_db_session();
}

pub struct Connection;

pub trait IConnection {
  fn init(&self) -> &'static Client;
}

impl IConnection for Connection {
  fn init(&self) -> &'static CurrentSession {
    lazy_static::initialize(&session);
    &*session
  }
}
