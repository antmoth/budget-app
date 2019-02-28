use rocket::request::LenientForm;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use MainDbConn;
use diesel::Connection;
use models::account::*;
use models::form_values::FormUuid;
use context::Context;
use error::Error;

#[get("/accounts")]
pub fn accounts(conn: MainDbConn, mut context: Context) -> Result<Template, Error> {
    let accounts = get_accounts(&conn)?;
    context.data = json!({ "accounts": &accounts });
    Ok(Template::render("accounts", context))
}

#[get("/new_account")]
pub fn new_account(conn: MainDbConn, context: Context) -> Template {
    Template::render("edit_account", context)
}

#[post("/new_account", data = "<account>")]
pub fn new_account_post(conn: MainDbConn, context: Context, account: LenientForm<FormAccount>) -> Result<Redirect, Error> {
    let account = account.into_inner();

    conn.transaction(|| {
        create_account(&conn, &account)?;
        Ok(Redirect::to("/accounts"))
    })
}

#[get("/edit_account/<id>")]
pub fn edit_account(conn: MainDbConn, mut context: Context, id: FormUuid) -> Result<Template, Error> {
    let (ref account, ref transactions) = get_account(&conn, id.0)?[0];

    context.data = json!({ "account": &account,
        "transactions": &transactions});
    Ok(Template::render("edit_account", context))
}

#[post("/edit_account/<id>", data = "<account>")]
pub fn edit_account_post(conn: MainDbConn, context: Context, id: FormUuid, account: LenientForm<FormAccount>) -> Result<Redirect, Error> {
    let account = account.into_inner();

    conn.transaction(|| {
        update_account(&conn, id.0, &account)?;
        Ok(Redirect::to(format!("/edit_account/{}", id)))
    })
    .or_else(|e| Err(e))
}
