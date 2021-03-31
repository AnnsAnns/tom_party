use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use redis::Commands;

use uuid::Uuid;

use rand::{distributions::Alphanumeric, Rng};

use crate::db;
use crate::helpers;

const MAXPLAYERS: u8 = 8;

pub mod create_invite;
pub mod init;
pub mod join_game;
