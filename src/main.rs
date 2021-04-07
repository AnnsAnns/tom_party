/*
* Copyright 2021 tomGER, git@tomger.eu
*
* Licensed under the EUPL, Version 1.2 or – as soon they will be approved by the European Commission - subsequent versions of the EUPL (the "Licence");
* You may not use this work except in compliance with theLicence.
*
* You may obtain a copy of the Licence at: https://joinup.ec.europa.eu/software/page/eupl
*
* Unless required by applicable law or agreed to in writing, software distributed under the Licence is distributed on an "AS IS" basis,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the Licence for the specific language governing permissions and limitations under the Licence.
*/

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
                games::svar::question::next_question,
                games::svar::answer::answer
            ],
        )
        .launch();
}
