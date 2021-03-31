#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod game;
mod helpers;
mod db;

fn main() {
    rocket::ignite().mount("/", routes![game::init]).launch();
}