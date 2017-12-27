use diesel::pg::PgConnection;
use diesel::{self, Connection, RunQueryDsl};
use rocket::response::Redirect;
use rocket::request::LenientForm;

use models::transaction::*;
use error::Error;
use context::Context;

#[post("/new_transaction", data = "<transaction>")]
pub fn new_transaction_post(context: Context, transaction: LenientForm<FormTransaction>) -> Result<Redirect, Error> {
    let transaction = transaction.get();

    context.db.transaction(|| {
        create_transaction(&context.db, &transaction);
        Ok(Redirect::to("/budget"))
    })
    .or_else(|e| Err(e))
}

pub fn create_transaction<'a>(conn: &PgConnection, transaction: &FormTransaction) -> Transaction {
    use schema::transactions;

    let new_transaction = NewTransaction {
        date: transaction.date.0,
        account: transaction.account.0,
        category: match transaction.category {
            Some(ref u) => Some(u.0),
            _ => None,
        },
        payee: match transaction.payee {
            Some(ref u) => Some(u.0),
            _ => None,
        },
        parent_transaction: match transaction.parent_transaction {
            Some(ref u) => Some(u.0),
            _ => None,
        },
        amount: transaction.amount.0.clone(),
        memo: match transaction.memo {
            Some(ref s) => Some(&s),
            _ => None
        },
        cleared: transaction.cleared,
    };

    diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .get_result(conn)
        .expect("Error saving new transaction")
}
