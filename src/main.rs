#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate budget_app;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    budget_app::start();
}
