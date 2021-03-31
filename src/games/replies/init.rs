use rocket_contrib::json::JsonValue;

use uuid::Uuid;

use crate::db;
use crate::helpers;

#[post("/init")]
pub fn init() -> JsonValue {
    let uuid_game = Uuid::new_v4().to_string();
    let uuid_owner = Uuid::new_v4().to_string();

    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_) => return helpers::error_message("Issue connecting to database"),
    };

    db::hset(
        &mut con,
        &format!("replies:{id}", id = &uuid_game),
        "owner",
        &uuid_owner,
    );

    db::hset(&mut con,&format!("replies:{}", &uuid_game) , "players", "0");

    json!({
        "worked": true,
        "uuid_game": uuid_game,
        "uuid_owner": uuid_owner
    })
}
