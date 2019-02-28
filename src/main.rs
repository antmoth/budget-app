#![feature(plugin)]

extern crate budget_app;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    budget_app::start();
}
