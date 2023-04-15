#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket_dyn_templates::{Template, context};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    status: String,
    message: String,
}

#[get("/")]
fn hello() -> &'static str {
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

#[get("/index")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            field: "value"
        }
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, index, todo])
        .attach(Template::fairing())
}

