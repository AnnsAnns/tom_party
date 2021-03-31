extern crate redis;
use redis::{Commands, Connection};
use rocket_contrib::json::JsonValue;
use std::error::Error;

use crate::helpers;

pub fn init_con() -> Result<redis::Connection, Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;

    Ok(con)
}

pub fn hset(con: &mut Connection, key: &str, field: &str, value: &str) {
    let _: () = con.hset(key, field, value).unwrap();
}

pub fn set(con: &mut Connection, key: &str, value: &str) {
    let _: () = con.set(key, value).unwrap();
}

pub fn replace(con: &mut Connection, old_key: &str, new_key: &str) {
    let _: () = con.set(old_key, new_key).unwrap();
}

pub fn find_invite(con: &mut Connection, invite_code: &str) -> Result<String, JsonValue> {
    let exists: bool = con.exists(invite_code.clone()).unwrap();

    if !exists {
        Err(helpers::error_message("Room doesn't exist"))
    } else {
        let session_id: String = con.get(invite_code).unwrap();
        Ok(session_id)
    }
}
