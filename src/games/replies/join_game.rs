use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use crate::helpers;

#[derive(Deserialize)]
pub struct InviteBody {
    uuid_game: String,
    uuid_owner: String,
}

#[post("/join_game", format = "json", data = "<data>")]
pub fn join_game(data: Json<InviteBody>) -> JsonValue {
    helpers::error_message("Not Implemented!")
}
