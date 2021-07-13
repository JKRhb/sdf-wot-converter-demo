#[macro_use]
extern crate rocket;

use rocket::form::{Context, Contextual, Form, FromForm};
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use sdf_wot_converter::converter;
use serde::Serialize;

use rocket_dyn_templates::Template;

#[derive(FromForm, Debug, Serialize)]
struct UserInput {
    input1: String,
    input2: String,
}

#[derive(FromForm, Debug, Serialize)]
struct Output {
    input1: String,
    input2: String,
    error: String,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &Context::default())
}

#[post("/", data = "<form>")]
fn submit(form: Form<Contextual<UserInput>>) -> (Status, Template) {
    let template = match form.value {
        Some(ref submission) => {
            println!("submission: {:#?}", submission);
            let sdf_input = submission.input1.clone();
            let wot_output: String;
            let error: String;
            match converter::convert_sdf_to_wot_tm(sdf_input.clone()) {
                Ok(result) => {
                    wot_output = result;
                    error = String::new();
                }
                Err(err) => {
                    wot_output = submission.input2.clone();
                    error = err.to_string();
                }
            }
            let output = Output {
                input1: sdf_input,
                input2: wot_output,
                error,
            };
            Template::render("index", &output)
        }
        None => Template::render("index", &form.context),
    };

    (form.context.status(), template)
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
}
