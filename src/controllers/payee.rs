use diesel::Connection;
use rocket::response::Redirect;
use rocket::request::LenientForm;
use rocket_contrib::Template;

use models::payee::{self, FormPayee};
use models::category;
use error::Error;
use context::Context;

#[get("/payees")]
pub fn payees(mut context: Context) -> Result<Template, Error> {
    let payees = payee::get_payees(&context.db);

    context.data = json!({ "payees": &payees });
    Ok(Template::render("payees", context))
}

#[get("/new_payee")]
pub fn new_payee(mut context: Context) -> Template {
    let categories = category::get_categories(&context.db);
    context.data = json!({ "categories": &categories });
    Template::render("edit_payee", context)
}

#[post("/new_payee", data="<payee>")]
pub fn new_payee_post(context: Context, payee: LenientForm<FormPayee>) -> Result<Redirect, Error> {
    let payee = payee.get();

    context.db.transaction(|| {
        payee::create_payee(&context.db, &payee);
        Ok(Redirect::to("/budget"))
    })
    .or_else(|e| Err(e))
}
