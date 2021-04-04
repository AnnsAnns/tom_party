#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod db;
mod games;
mod helpers;

fn main() {
    rocket::ignite()
        .mount(
            "/games/lobby/",
            routes![
                games::lobby::init::init,
                games::lobby::create_invite::create_invite,
                games::lobby::join_game::join_game,
                games::lobby::request_player_data::request_player_data,
                games::lobby::heartbeat::heartbeat
            ],
        )
        .mount(
            "/games/svar/",
            routes![
                games::svar::question::get_question,
                games::svar::question::next_question
            ],
        )
        .launch();
}
