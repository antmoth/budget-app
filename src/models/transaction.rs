use diesel::pg::PgConnection;
use diesel::{self, RunQueryDsl};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};

use schema::transactions;
use models::form_values::*;
use models::account::Account;

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Account)]
pub struct Transaction {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub date: NaiveDate,
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,
    pub payee_id: Option<Uuid>,
    pub parent_transaction_id: Option<Uuid>,
    pub amount: BigDecimal,
    pub memo: Option<String>,
    pub cleared: bool,
}

#[derive(Insertable)]
#[table_name="transactions"]
pub struct NewTransaction<'a> {
    pub date: NaiveDate,
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,
    pub payee_id: Option<Uuid>,
    pub parent_transaction_id: Option<Uuid>,
    pub amount: BigDecimal,
    pub memo: Option<&'a str>,
    pub cleared: bool,
}

#[derive(FromForm)]
pub struct FormTransaction {
    pub date: FormDate,
    pub account_id: FormUuid,
    pub category_id: Option<FormUuid>,
    pub payee_id: Option<FormUuid>,
    pub parent_transaction_id: Option<FormUuid>,
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
        account_id: transaction.account_id.0,
        category_id: match transaction.category_id {
            Some(ref cid) => Some(cid.0),
            _ => None,
        },
        payee_id: match transaction.payee_id {
            Some(ref pid) => Some(pid.0),
            _ => None,
        },
        parent_transaction_id: match transaction.parent_transaction_id {
            Some(ref tid) => Some(tid.0),
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
