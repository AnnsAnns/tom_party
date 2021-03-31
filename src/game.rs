use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use redis::Commands;

use uuid::Uuid;

use crate::db;
use crate::helpers;

mod replies;

#[derive(Deserialize)]
pub struct InitStructure {
    action: String,
    game_id: Option<String>,
}

#[post("/game/replies", format = "json", data = "<data>")]
pub fn init(data: Json<InitStructure>) -> JsonValue {
    match data.action.as_str() {
        "init" => replies::init(data),
        _ => helpers::error_message("Couldn't find action")
    }
}