#![deny(rust_2018_idioms)]
#![deny(clippy::correctness)]
#![deny(clippy::perf)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub mod model;
pub mod routes;
pub mod scheduler;
pub mod ticker;

pub type AppState = Arc<Mutex<HashMap<u8, (bool, i16)>>>;
