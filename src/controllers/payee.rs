use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::{self, RunQueryDsl};
use rocket::response::Redirect;
use rocket::request::LenientForm;

use models::payee::*;
use error::Error;
use context::Context;

#[post("/new_payee", data="<payee>")]
pub fn new_payee_post(context: Context, payee: LenientForm<FormPayee>) -> Result<Redirect, Error> {
    let payee = payee.get();

    context.db.transaction(|| {
        create_payee(&context.db, &payee);
        Ok(Redirect::to("/budget"))
    })
    .or_else(|e| Err(e))
}

pub fn create_payee<'a>(conn: &PgConnection, payee: &FormPayee) -> Payee {
    use schema::payees;

    let new_payee = NewPayee {
        name: &payee.name,
        default_category: match payee.default_category {
            Some(ref u) => Some(u.0),
            _ => None,
        }
    };

    diesel::insert_into(payees::table)
        .values(&new_payee)
        .get_result(conn)
        .expect("Error saving new payee")
}

