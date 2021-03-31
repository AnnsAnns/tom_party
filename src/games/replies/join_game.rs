use redis::Commands;
use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use crate::db;
use crate::helpers;

#[derive(Deserialize)]
pub struct JoinGame {
    invite_code: String,
    name: String
}

#[post("/join_game", format = "json", data = "<data>")]
pub fn join_game(data: Json<JoinGame>) -> JsonValue {
    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_) => return helpers::error_message("Issue connecting to database"),
    };

    let session_id = match db::find_invite(&mut con, &data.invite_code) {
        Ok(session_id) => session_id,
        Err(err) => return err
    };

    let players: u8 = con.hget(&format!("replies:{}", &session_id), "players").unwrap();

    db::hset(&mut con, &format!("replies:{}", &session_id), "players", &(players + 1).to_string()); // Increase active players by one

    json!({
        "worked": true,
        "session_id": session_id,
        "username": data.name
    })
}
