#[macro_use]
extern crate rocket;

use chrono::{DateTime, Utc};
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct TimeMessage {
    time: String,
}

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

#[get("/get_time")]
fn get_time() -> Json<TimeMessage> {
    let current_time: DateTime<Utc> = Utc::now();
    let time_str = current_time.format("%H:%M:%S").to_string();
    Json(TimeMessage { time: time_str })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, get_time])
}
