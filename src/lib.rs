#![recursion_limit="128"]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate uuid;
extern crate bigdecimal;
extern crate chrono;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;

use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_category<'a>(conn: &PgConnection, name: &'a str, allocated: Option<BigDecimal>, parent_category: Option<Category>) -> Category {
    use schema::categories;

    let new_category = NewCategory {
        name: name,
        allocated: allocated,
        parent_category: match parent_category {
            Some(cat) => Some(cat.id),
            _ => None,
        },
    };

    diesel::insert(&new_category).into(categories::table)
        .get_result(conn)
        .expect("Error saving new  category")
}

pub fn create_account<'a>(conn: &PgConnection, name: &'a str, cleared_balance: BigDecimal, uncleared_balance: BigDecimal, on_budget: bool) -> Account {
    use schema::accounts;

    let new_account = NewAccount {
        name: name,
        cleared_balance: cleared_balance,
        uncleared_balance: uncleared_balance,
        on_budget: on_budget,
    };

    diesel::insert(&new_account).into(accounts::table)
        .get_result(conn)
        .expect("Error saving new account")
}

pub fn create_payee<'a>(conn: &PgConnection, name: &'a str, default_category: Option<Category>) -> Payee {
    use schema::payees;

    let new_payee = NewPayee {
        name: name,
        default_category: match default_category {
            Some(cat) => Some(cat.id),
            _ => None,
        }
    };

    diesel::insert(&new_payee).into(payees::table)
        .get_result(conn)
        .expect("Error saving new payee")
}

pub fn create_transaction<'a>(conn: &PgConnection, date: NaiveDate, account: Account, category: Option<Category>, payee: Option<Payee>, parent_transaction: Option<Transaction>, amount: BigDecimal, memo: Option<&'a str>, cleared: bool) -> Transaction {
    use schema::transactions;

    let new_transaction = NewTransaction {
        date: date,
        account: account.id,
        category: match category {
            Some(cat) => Some(cat.id),
            _ => None,
        },
        payee: match payee {
            Some(pay) => Some(pay.id),
            _ => None,
        },
        parent_transaction: match parent_transaction {
            Some(tran) => Some(tran.id),
            _ => None,
        },
        amount: amount,
        memo: memo,
        cleared: cleared,
    };

    diesel::insert(&new_transaction).into(transactions::table)
        .get_result(conn)
        .expect("Error saving new transaction")
}
