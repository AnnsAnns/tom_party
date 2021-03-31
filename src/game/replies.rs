use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use redis::Commands;

use uuid::Uuid;

use crate::db;

#[post("/init")]
pub fn init() -> JsonValue {
    let uuid_game = Uuid::new_v4().to_string();
    let uuid_user = Uuid::new_v4().to_string();

    let mut con = db::init_con().unwrap(); // @TODO: Proper error handling

    let _ : () = con.hset(format!("replies:{id}", id = &uuid_game),"owner", &uuid_user).unwrap();

    json!({
        "worked": true,
        "uuid_game": uuid_game,
        "uuid_user": uuid_user
    })
}