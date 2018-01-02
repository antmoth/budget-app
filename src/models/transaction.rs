use diesel::pg::PgConnection;
use diesel::{self, RunQueryDsl};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;

use schema::transactions;
use models::form_values::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub date: NaiveDate,
    pub account: Uuid,
    pub category: Option<Uuid>,
    pub payee: Option<Uuid>,
    pub parent_transaction: Option<Uuid>,
    pub amount: BigDecimal,
    pub memo: Option<String>,
    pub cleared: bool,
}

#[derive(Insertable)]
#[table_name="transactions"]
pub struct NewTransaction<'a> {
    pub date: NaiveDate,
    pub account: Uuid,
    pub category: Option<Uuid>,
    pub payee: Option<Uuid>,
    pub parent_transaction: Option<Uuid>,
    pub amount: BigDecimal,
    pub memo: Option<&'a str>,
    pub cleared: bool,
}

#[derive(FromForm)]
pub struct FormTransaction {
    pub date: FormDate,
    pub account: FormUuid,
    pub category: Option<FormUuid>,
    pub payee: Option<FormUuid>,
    pub parent_transaction: Option<FormUuid>,
    pub amount: FormDecimal,
    pub memo: Option<String>,
    pub cleared: bool,
}

pub fn get_transactions(conn: &PgConnection) -> Vec<Transaction> {
    use schema::transactions::dsl::*;

    transactions
        .load::<Transaction>(conn)
        .expect("Error loading transactions")
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
