#![recursion_limit="128"]
#![feature(plugin, custom_attribute, try_trait, decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate dotenv;
extern crate uuid;
extern crate bigdecimal;
extern crate chrono;
extern crate num_traits;
extern crate serde;
#[macro_use] extern crate serde_json;

mod schema;
mod models;
mod controllers;
mod context;
mod error;

use dotenv::dotenv;
use rocket::Rocket;
use rocket_contrib::templates::Template;

#[database("main_db")]
pub struct MainDbConn(rocket_contrib::databases::diesel::PgConnection);

pub fn ignite() -> Rocket {
    dotenv().ok();

    let r = rocket::ignite()
        .attach(Template::fairing())
        .attach(MainDbConn::fairing());
    
    controllers::mount(r)
}

pub fn start() {
    ignite().launch();
}
