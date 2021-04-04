use std::collections::HashMap;

use redis::Commands;
use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::db;
use crate::helpers;

#[derive(Deserialize)]
pub struct RequestPlayerData {
    uuid_game: String,
}

#[post("/request_player_data", format = "json", data = "<data>")]
pub fn request_player_data(data: Json<RequestPlayerData>) -> JsonValue {
    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_err) => return helpers::error_message("Couldn't connect to database!")
    };

    let players: Vec<String> = con.hkeys(format!("{}:players", &data.uuid_game)).unwrap();

    let mut response_data: HashMap<String, HashMap<String, String>> = HashMap::new();

    for player in players.iter() {
        if player == "amount" { // Ignore non-player in DB
            continue
        }

        let player_data: HashMap<String, String> = con.hgetall(format!("{}:players:{}", &data.uuid_game, player)).unwrap();

        response_data.insert(String::from(player), player_data);
    };

    json!(response_data)
}