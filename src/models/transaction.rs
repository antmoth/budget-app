use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use schema::transactions;

#[derive(Queryable)]
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
