use diesel::pg::PgConnection;
use diesel::{self, Connection, RunQueryDsl};
use rocket::response::Redirect;
use rocket::request::LenientForm;
use rocket_contrib::Template;

use models::transaction::*;
use error::Error;
use context::Context;

#[get("/transactions")]
pub fn transactions(mut context: Context) -> Result<Template, Error> {
    let transactions = get_transactions(&context.db);
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
        create_transaction(&context.db, &transaction);
        Ok(Redirect::to("/budget"))
    })
    .or_else(|e| Err(e))
}

fn get_transactions(conn: &PgConnection) -> Vec<Transaction> {
    use schema::transactions::dsl::*;

    transactions
        .load::<Transaction>(conn)
        .expect("Error loading transactions")
}

fn create_transaction<'a>(conn: &PgConnection, transaction: &FormTransaction) -> Transaction {
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
