use rand::{Rng, thread_rng};
use redis::Commands;
use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::db;
use crate::helpers;

#[derive(Deserialize)]
pub struct Answer {
    uuid_game: String,
    answer: String,
    question_id: String,
    user_id: String,
    username: String
}

#[post("/answer", format = "json", data = "<data>")]
pub fn answer(data: Json<Answer>) -> JsonValue {
    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_err) => return helpers::error_message("Couldn't connect to database!")
    };

    let user_id: String = con.hget(&format!("{}:players", &data.uuid_game), &data.username).unwrap();

    if user_id != data.user_id {
        return helpers::error_message("User ID did not match!")
    }

    if con.hexists(format!("{}:svar:{}:answer", &data.uuid_game, &data.question_id), &data.username).unwrap() {
        return helpers::error_message("Already submitted your answer!")
    }

    db::hset(&mut con, &format!("{}:svar:{}:answer", &data.uuid_game, &data.question_id), &data.username, &data.answer, 10);

    json!({
        "worked": true,
    })
}