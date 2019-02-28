use rocket::response::Redirect;
use rocket::request::LenientForm;
use rocket_contrib::templates::Template;

use crate::MainDbConn;
use diesel::Connection;
use crate::models::transaction::{self, FormTransaction};
use crate::error::Error;
use crate::context::Context;

#[get("/transactions")]
pub fn transactions(conn: MainDbConn, mut context: Context) -> Result<Template, Error> {
    let transactions = transaction::get_transactions(&conn);
    context.data = json!({ "transactions": &transactions });
    Ok(Template::render("transactions", context))
}

#[get("/new_transaction")]
pub fn new_transaction(context: Context) -> Template {
    Template::render("edit_transaction", context)
}

#[post("/new_transaction", data = "<transaction>")]
pub fn new_transaction_post(conn: MainDbConn, _context: Context, transaction: LenientForm<FormTransaction>) -> Result<Redirect, Error> {
    let transaction = transaction.into_inner();

    conn.transaction(|| {
        transaction::create_transaction(&conn, &transaction);
        Ok(Redirect::to("/budget"))
    })
    .or_else(|e| Err(e))
}
