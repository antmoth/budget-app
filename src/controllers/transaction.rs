use diesel::pg::PgConnection;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{self, LoadDsl};

use models::account::*;
use models::category::*;
use models::payee::*;
use models::transaction::*;

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
