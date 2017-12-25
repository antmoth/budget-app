use diesel::pg::PgConnection;
use bigdecimal::BigDecimal;
use diesel::{self, RunQueryDsl};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use std::collections::HashMap;
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
    let account = account.get();

    Redirect::to("/accounts")
}

fn create_account<'a>(conn: &PgConnection, account: FormAccount) -> Account {
    use schema::accounts;

    let new_account = NewAccount {
        name: &account.name,
        cleared_balance: account.balance.0,
        uncleared_balance: BigDecimal::zero(),
        on_budget: account.on_budget,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Error saving new account")
}
