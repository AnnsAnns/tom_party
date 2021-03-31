use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use redis::Commands;

use uuid::Uuid;

use crate::db;

#[post("/init")]
pub fn init() -> JsonValue {
    // @TODO: Verify session
    // @TODO: Verify data
    let uuid = Uuid::new_v4();

    let mut con = db::init_con().unwrap(); // @TODO: Proper error handling

    let _ : () = con.hset(format!("replies:{id}", id = uuid),"alive", true).unwrap();

    let game_info: Option<String> = con.hget(format!("replies:{id}", id = uuid), "alive").unwrap();

    json!({
        "worked": true,
        "redis_data": game_info,
        "id": uuid,
    })
}
