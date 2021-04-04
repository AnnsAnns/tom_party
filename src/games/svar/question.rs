use rand::{Rng, thread_rng};
use redis::Commands;
use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::db;
use crate::helpers;

#[derive(Deserialize)]
pub struct GetQuestion {
    uuid_game: String,
}

#[post("/get_question", format = "json", data = "<data>")]
pub fn get_question(data: Json<GetQuestion>) -> JsonValue {
    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_err) => return helpers::error_message("Couldn't connect to database!")
    };

    let question: String = match con.hget(format!("{}:svar", &data.uuid_game), "current_question") {
        Ok(question) => question,
        Err(_err) => return helpers::error_message("Couldn't get question!")
    };

    json!({
        "worked": true,
        "question": question
    })
}

#[derive(Deserialize)]
pub struct NextQuestion {
    uuid_game: String,
    user_id: String
}

#[post("/next_question", format = "json", data = "<data>")]
pub fn next_question(data: Json<NextQuestion>) -> JsonValue {
    let questions: Vec<&str> = vec![
        "A Gopher and Crab meet, the crab says: ______",
        "Top Text, ______"
    ];

    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_err) => return helpers::error_message("Couldn't connect to database!")
    };

    let owner_id: String = con.hget(&data.uuid_game, "owner").unwrap();

    if owner_id != data.user_id {
        return helpers::error_message("User is not the owner, insufficient permissions!")
    };

    let mut rng = thread_rng();
    let question: String = questions[rng.gen_range(0..questions.len())].to_string();

    json!({
        "worked": true,
        "question": question
    })
}
