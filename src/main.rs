#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate budget_app;

use std::collections::HashMap;
use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("greeting", "Hello");
    Template::render("index", context)
}

fn main() {
    let conn = budget_app::establish_connection();
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
