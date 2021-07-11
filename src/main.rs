#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use sdf_wot_converter::converter;

#[derive(FromForm)]
struct UserInput {
    value: String,
}

#[post("/", data = "<user_input>")]
fn submit(user_input: Form<UserInput>) -> String {
    if let Ok(result) = converter::convert_sdf_to_wot_tm(user_input.value.clone()) {
        result
    } else {
        "Something went wrong!".to_string()
    }
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![submit])
}
