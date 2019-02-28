use diesel::pg::PgConnection;
use diesel::{self, RunQueryDsl};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};

use crate::schema::transactions;
use crate::models::form_values::*;
use crate::models::account::Account;

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Account)]
pub struct Transaction {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub date: NaiveDate,
    pub account_id: Uuid,
    pub amount: BigDecimal,
    pub memo: Option<String>,
}

#[derive(Insertable)]
#[table_name="transactions"]
pub struct NewTransaction<'a> {
    pub date: NaiveDate,
    pub account_id: Uuid,
    pub amount: BigDecimal,
    pub memo: Option<&'a str>,
}

#[derive(FromForm)]
pub struct FormTransaction {
    pub date: FormDate,
    pub account_id: FormUuid,
    pub amount: FormDecimal,
    pub memo: Option<String>,
}

pub fn get_transactions(conn: &PgConnection) -> Vec<Transaction> {
    use crate::schema::transactions::dsl::*;

    transactions
        .load::<Transaction>(conn)
        .expect("Error loading transactions")
}

pub fn create_transaction<'a>(conn: &PgConnection, transaction: &FormTransaction) -> Transaction {
    use crate::schema::transactions;

    let new_transaction = NewTransaction {
        date: transaction.date.0,
        account_id: transaction.account_id.0,
        amount: transaction.amount.0.clone(),
        memo: match transaction.memo {
            Some(ref s) => Some(&s),
            _ => None
        },
    };

    diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .get_result(conn)
        .expect("Error saving new transaction")
}
