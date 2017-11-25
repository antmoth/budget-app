use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;

#[derive(Queryable)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub allocated: Option<BigDecimal>,
    pub parent_category: Uuid,
}

pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub cleared_balance: BigDecimal,
    pub uncleared_balance: BigDecimal,
    pub on_budget: bool,
}

pub struct Payee {
    pub id: Uuid,
    pub name: String,
    pub default_category: Uuid,
}

pub struct Transaction {
    pub id: Uuid,
    pub date: NaiveDate,
    pub account: Option<Uuid>,
    pub category: Option<Uuid>,
    pub payee: Option<Uuid>,
    pub parent_transaction: Option<Uuid>,
    pub amount: BigDecimal,
    pub memo: String,
    pub cleared: bool,
}
