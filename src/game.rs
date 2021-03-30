use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use redis::Commands;

use uuid::Uuid;

use crate::db;

#[derive(Deserialize)]
pub struct InitStructure {
    game_type: String,
}

#[post("/game", format = "json", data = "<data>")]
pub fn init(data: Json<InitStructure>) -> JsonValue {
    // @TODO: Verify session
    // @TODO: Verify data

    let uuid = Uuid::new_v4();

    let mut con = db::init_con().unwrap(); // @TODO: Proper error handling

    let _ : () = con.hset(format!("{game_type}:{id}", game_type = data.game_type, id = uuid),"alive", true).unwrap();

    let game_info: Option<String> = con.hget(format!("{game_type}:{id}", game_type = data.game_type, id = uuid), "alive").unwrap();

    json!({
        "worked": true,
        "description": data.game_type,
        "redis_data": game_info,
        "id": uuid,
    })
}
