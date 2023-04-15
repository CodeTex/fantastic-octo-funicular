#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    status: String,
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/todo")]
fn todo() -> Json<Task> {
    let task = Task {
        status: String::from("Success"),
        message: String::from("Test message")
    };
    Json(task)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, todo])
}

