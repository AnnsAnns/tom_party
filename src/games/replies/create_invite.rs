use std::borrow::Borrow;

use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use redis::Commands;

use rand::{distributions::Alphanumeric, Rng};

use crate::db;
use crate::helpers;

#[derive(Deserialize)]
pub struct InviteBody {
    uuid_game: String,
    uuid_owner: String,
    old_token: Option<String>,
}

#[post("/create_invite", format = "json", data = "<data>")]
pub fn create_invite(data: Json<InviteBody>) -> JsonValue {
    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_) => return helpers::error_message("Issue connecting to database"),
    };

    let owner: String = match con.hget(format!("replies:{}", data.uuid_game), "owner") {
        Ok(string) => string,
        Err(err) => return helpers::error_message("Error finding match!"),
    };

    if owner != data.uuid_owner {
        return helpers::error_message("Owner UUID is not the same, no permissions!");
    }

    let invite_code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect(); // Stolen from https://stackoverflow.com/questions/54275459/how-do-i-create-a-random-string-by-sampling-from-alphanumeric-characters

    db::hset(
        &mut con,
        &format!("replies:{}", &data.uuid_game),
        "invite_code",
        &invite_code,
    );

    match &data.old_token {
        // Only rename old session if it exists
        Some(old_token) => db::rename(&mut con, &old_token, &invite_code),
        None => db::set(&mut con, &invite_code, &data.uuid_game),
    }

    json!({
        "worked": true,
        "uuid_game": data.uuid_game,
        "invite_code": invite_code
    })
}
