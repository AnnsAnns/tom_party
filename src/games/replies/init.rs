use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use redis::Commands;

use uuid::Uuid;

use rand::{distributions::Alphanumeric, Rng};

use crate::db;
use crate::helpers;

#[post("/init")]
pub fn init() -> JsonValue {
    let uuid_game = Uuid::new_v4().to_string();
    let uuid_owner = Uuid::new_v4().to_string();

    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(err) => return helpers::error_message("Issue connecting to database"),
    };

    db::set(
        con,
        &format!("replies:{id}", id = &uuid_game),
        "owner",
        &uuid_owner,
    );

    json!({
        "worked": true,
        "uuid_game": uuid_game,
        "uuid_owner": uuid_owner
    })
}
