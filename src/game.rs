use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;


#[derive(Deserialize)]
pub struct Task {
    description: String,
}

#[post("/game", format = "json", data = "<task>")]
pub fn index(task: Json<Task>) -> JsonValue {
    
    json!({
        "worked": true,
        "description": task.description
    })
}
