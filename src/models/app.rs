use std::sync::Arc;
use crate::{CurrentSession};

pub struct AppState {
  pub session: Arc<CurrentSession>,
}

