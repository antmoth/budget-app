use rocket::Rocket;
use rocket_contrib::Template;
use std::collections::HashMap;

pub mod account;
pub mod category;
pub mod payee;
pub mod transaction;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("greeting", "Hello");
    Template::render("index", context)
}

#[get("/budget")]
fn budget() -> Template {
    let mut context = HashMap::new();
    context.insert("dummy", "dummy");
    Template::render("budget", context)
}

#[get("/accounts")]
fn accounts() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("accounts", context)
}

pub fn mount(r: Rocket) -> Rocket {
    r
        .mount("/", [
               routes![index, budget, accounts]
        ].concat())
}
