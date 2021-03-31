extern crate redis;
use std::error::Error;
use redis::{Commands, RedisResult, Connection};

pub fn init_con() -> Result<redis::Connection, Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;

    Ok(con)
}

pub fn set(mut con: Connection, key: &str, field: &str, value: &str) {
    let _ : () = con.hset(key, field, value).unwrap();
}