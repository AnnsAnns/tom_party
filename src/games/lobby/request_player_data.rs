use std::collections::HashMap;

use redis::Commands;
use rocket::{Request, response};
use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use uuid::Uuid;

use crate::db;
use crate::helpers;

#[derive(Deserialize)]
pub struct RequestPlayerData {
    session_id: String,
}

#[post("/request_player_data", format = "json", data = "<data>")]
pub fn request_player_data(data: Json<RequestPlayerData>) -> JsonValue {
    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_err) => return helpers::error_message("Couldn't connect to database!")
    };

    let players: Vec<String> = con.hkeys(format!("{}:players", &data.session_id)).unwrap();

    let mut response_data: HashMap<String, HashMap<String, String>> = HashMap::new();

    for player in players.iter() {
        if player == "amount" { // Ignore non-player in DB
            continue
        }

        let player_data: HashMap<String, String> = con.hgetall(format!("{}:players:{}", &data.session_id, player)).unwrap();

        response_data.insert(String::from(player), player_data);
    };

    json!(response_data)
}