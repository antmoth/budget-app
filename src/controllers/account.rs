use diesel::pg::PgConnection;
use bigdecimal::BigDecimal;
use diesel::{self, RunQueryDsl};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use std::collections::HashMap;
use std::str::FromStr;
use num_traits::identities::Zero;

use models::account::*;

#[get("/accounts")]
pub fn accounts() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("accounts", context)
}

#[get("/new_account")]
pub fn new_account() -> Template {
    Template::render("edit_account", HashMap::<String, String>::new())
}

#[post("/new_account", data = "<account>")]
pub fn new_account_post(account: Form<FormAccount>) -> Redirect {

    Redirect::to("/accounts")
}

fn create_account<'a>(conn: &PgConnection, name: &'a str, balance: BigDecimal, on_budget: bool) -> Account {
    use schema::accounts;

    let new_account = NewAccount {
        name: name,
        cleared_balance: balance,
        uncleared_balance: BigDecimal::zero(),
        on_budget: on_budget,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Error saving new account")
}
