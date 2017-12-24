use diesel::pg::PgConnection;
use bigdecimal::BigDecimal;
use diesel::{self, RunQueryDsl};

use models::account::*;

pub fn create_account<'a>(conn: &PgConnection, name: &'a str, cleared_balance: BigDecimal, uncleared_balance: BigDecimal, on_budget: bool) -> Account {
    use schema::accounts;

    let new_account = NewAccount {
        name: name,
        cleared_balance: cleared_balance,
        uncleared_balance: uncleared_balance,
        on_budget: on_budget,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Error saving new account")
}
