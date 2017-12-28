use diesel::pg::PgConnection;
use diesel::Connection;
use bigdecimal::BigDecimal;
use diesel::{self, RunQueryDsl};
use rocket::request::LenientForm;
use rocket::response::Redirect;
use rocket_contrib::Template;
use num_traits::identities::Zero;

use models::account::*;
use context::Context;
use error::Error;

#[get("/accounts")]
pub fn accounts(mut context: Context) -> Result<Template, Error> {
    let accounts = get_accounts(&context.db);
    context.data = json!({ "accounts": &accounts });
    Ok(Template::render("accounts", context))
}

#[get("/new_account")]
pub fn new_account(context: Context) -> Template {
    Template::render("edit_account", context)
}

#[post("/new_account", data = "<account>")]
pub fn new_account_post(context: Context, account: LenientForm<FormAccount>) -> Result<Redirect, Error> {
    let account = account.get();

    context.db.transaction(|| {
        create_account(&context.db, &account);
        Ok(Redirect::to("/accounts"))
    })
    .or_else(|e| Err(e))
}

fn get_accounts(conn: &PgConnection) -> Vec<Account> {
    use schema::accounts::dsl::*;

    accounts
        .load::<Account>(conn)
        .expect("Error loading accounts")
}

fn create_account<'a>(conn: &PgConnection, account: &FormAccount) -> Account {
    use schema::accounts;

    let new_account = NewAccount {
        name: &account.name,
        cleared_balance: account.balance.0.clone(),
        uncleared_balance: BigDecimal::zero(),
        on_budget: account.on_budget,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Error saving new account")
}
