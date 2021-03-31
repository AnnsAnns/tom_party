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
            "/games/replies/",
            routes![
                games::replies::init::init,
                games::replies::create_invite::create_invite
            ],
        )
        .launch();
}
