use redis::Commands;
use rocket::http::Status;
use serde::Deserialize;

use rocket_contrib::json::Json;
use crate::db;

#[derive(Deserialize)]
pub struct Heartbeat {
    uuid_game: String,
    user_id: String,
    username: String
}

#[post("/heartbeat", format = "json", data = "<data>")]
pub fn heartbeat(data: Json<Heartbeat>) -> Status {
    let mut con = match db::init_con() {
        Ok(con) => con,
        Err(_err) => return Status::InternalServerError
    };

    let uuid_user: String = match con.hget(format!("{}:players", &data.uuid_game), &data.username) {
        Ok(u) => u,
        Err(_err) => return Status::NotFound
    };

    if uuid_user != data.user_id {
        return Status::Forbidden
    };

    let _: () = con.expire(format!("{}:players:{}", &data.uuid_game, &data.username), 3 * 60).unwrap(); // Expire session after ? minutes

    Status::Ok
}