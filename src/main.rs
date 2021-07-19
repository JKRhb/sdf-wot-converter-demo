#[macro_use]
extern crate rocket;

use rocket::form::{Context, Contextual, Form, FromForm};
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::Request;
use sdf_wot_converter::converter;
use serde::Serialize;
use std::collections::HashMap;

use rocket_dyn_templates::Template;

#[derive(Debug, FromFormField, Serialize, Clone, Copy)]
enum InputTypes {
    Sdf,
    WoTTM,
}

#[derive(FromForm, Debug, Serialize)]
struct UserInput {
    submit_input1: Option<String>,
    submit_input2: Option<String>,
    input1_type: InputTypes,
    input2_type: InputTypes,
    input1: String,
    input2: String,
}

#[derive(FromForm, Debug, Serialize)]
struct Output {
    input1: String,
    input2: String,
    error: String,
}

enum Direction {
    Left,
    Right,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &Context::default())
}

#[post("/", data = "<form>")]
fn submit(form: Form<Contextual<UserInput>>) -> (Status, Template) {
    let template;

    if let Some(ref submission) = form.value {
        println!("submission: {:#?}", submission);
        let direction;
        if submission.submit_input1.is_some() {
            direction = Some(Direction::Right);
        } else if submission.submit_input2.is_some() {
            direction = Some(Direction::Left);
        } else {
            direction = None;
        }

        if let Some(direction) = direction {
            let input;
            let input_type;
            let mut output;
            let output_type;
            let error;
            let return_data;
            match direction {
                Direction::Right => {
                    input = submission.input1.clone();
                    output = submission.input2.clone();
                    input_type = submission.input1_type;
                    output_type = submission.input2_type;
                }
                Direction::Left => {
                    input = submission.input2.clone();
                    output = submission.input1.clone();
                    input_type = submission.input2_type;
                    output_type = submission.input1_type;
                }
            }
            match input_type {
                InputTypes::Sdf => match output_type {
                    InputTypes::Sdf => {
                        output = input.clone();
                        error = String::new();
                    }
                    InputTypes::WoTTM => match converter::convert_sdf_to_wot_tm(input.clone()) {
                        Ok(result) => {
                            output = result;
                            error = String::new();
                        }
                        Err(err) => {
                            error = err.to_string();
                        }
                    },
                },
                InputTypes::WoTTM => match output_type {
                    InputTypes::Sdf => match converter::convert_wot_tm_to_sdf(input.clone()) {
                        Ok(result) => {
                            output = result;
                            error = String::new();
                        }
                        Err(err) => {
                            error = err.to_string();
                        }
                    },
                    InputTypes::WoTTM => {
                        output = input.clone();
                        error = String::new();
                    }
                },
            }

            match direction {
                Direction::Right => {
                    return_data = Output {
                        input1: input,
                        input2: output,
                        error,
                    };
                }
                Direction::Left => {
                    return_data = Output {
                        input1: output,
                        input2: input,
                        error,
                    };
                }
            }

            template = Template::render("index", &return_data);
        } else {
            template = Template::render("index", &form.context);
        }
    } else {
        template = Template::render("index", &form.context);
    }

    (form.context.status(), template)
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let mut context = HashMap::new();
    context.insert("uri", req.uri());
    Template::render("error/404", context)
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
        .register("/", catchers![not_found])
}
