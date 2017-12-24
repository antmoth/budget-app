use rocket::Rocket;
use rocket_contrib::Template;
use std::collections::HashMap;

mod account;
mod category;
mod payee;
mod transaction;

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

pub fn mount(r: Rocket) -> Rocket {
    r
        .mount("/", [
               routes![index, budget,
                   account::accounts, account::new_account]
        ].concat())
}
