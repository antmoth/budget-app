use diesel::Connection;
use rocket::request::LenientForm;
use rocket::response::Redirect;
use rocket_contrib::Template;

use models::account::*;
use context::Context;
use error::Error;

#[get("/accounts")]
pub fn accounts(mut context: Context) -> Result<Template, Error> {
    let accounts = get_accounts(&context.db)?;
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
