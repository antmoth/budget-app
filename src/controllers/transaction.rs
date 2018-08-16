use rocket::response::Redirect;
use rocket::request::LenientForm;
use rocket_contrib::Template;

use models::transaction::{self, FormTransaction};
use error::Error;
use context::Context;

#[get("/transactions")]
pub fn transactions(mut context: Context) -> Result<Template, Error> {
    let transactions = transaction::get_transactions(&context.db);
    context.data = json!({ "transactions": &transactions });
    Ok(Template::render("transactions", context))
}

#[get("/new_transaction")]
pub fn new_transaction(context: Context) -> Template {
    Template::render("edit_transaction", context)
}

#[post("/new_transaction", data = "<transaction>")]
pub fn new_transaction_post(context: Context, transaction: LenientForm<FormTransaction>) -> Result<Redirect, Error> {
    let transaction = transaction.get();

    context.db.transaction(|| {
        transaction::create_transaction(&context.db, &transaction);
        Ok(Redirect::to("/budget"))
    })
    .or_else(|e| Err(e))
}
