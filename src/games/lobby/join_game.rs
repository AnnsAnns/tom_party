use redis::Commands;
use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use uuid::Uuid;

use crate::db;
use crate::helpers;

use crate::games::lobby::MAXPLAYERS;

#[derive(Deserialize)]
pub struct JoinGame {
    invite_code: String,
    name: String,
}

#[post("/join_game", format = "json", data = "<data>")]
pub fn join_game(data: Json<JoinGame>) -> JsonValue {
    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_) => return helpers::error_message("Issue connecting to database"),
    };

    let session_id = match db::find_invite(&mut con, &data.invite_code) {
        // Search for room with invite code that matches
        Ok(session_id) => session_id,
        Err(err) => return err,
    };

    let player_names: Vec<String> = con
        .hkeys(format!("replies:{}:players", &session_id))
        .unwrap(); // Get all player names
    if player_names.contains(&data.name) {
        return helpers::error_message("Player name already exists!");
    }

    let players: u8 = con
        .hget(&format!("replies:{}:players", &session_id), "amount")
        .unwrap(); // Get amount of active players
    if players >= MAXPLAYERS {
        return helpers::error_message("Max player amount reached");
    }

    db::hset(
        &mut con,
        &format!("replies:{}:players", &session_id),
        "amount",
        &(&players + 1).to_string(),
    ); // Increase active players by one

    let user_id: String = match players {
        0 => con
            .hget(&format!("replies:{}", &session_id), "owner")
            .unwrap(),
        _ => Uuid::new_v4().to_string(),
    };

    db::hset(
        &mut con,
        &format!("replies:{}:players", &session_id),
        &data.name,
        &user_id,
    );

    json!({
        "worked": true,
        "session_id": session_id,
        "username": data.name,
        "user_id": user_id,
        "active_players": players + 1
    })
}
