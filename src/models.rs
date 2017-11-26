use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;

use super::schema::*;

#[derive(Queryable)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub allocated: Option<BigDecimal>,
    pub parent_category: Option<Uuid>,
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub allocated: Option<BigDecimal>,
    pub parent_category: Option<Uuid>,
}

#[derive(Queryable)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub cleared_balance: BigDecimal,
    pub uncleared_balance: BigDecimal,
    pub on_budget: bool,
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub cleared_balance: BigDecimal,
    pub uncleared_balance: BigDecimal,
    pub on_budget: bool,
}

#[derive(Queryable)]
pub struct Payee {
    pub id: Uuid,
    pub name: String,
    pub default_category: Option<Uuid>,
}

#[derive(Insertable)]
#[table_name="payees"]
pub struct NewPayee<'a> {
    pub name: &'a str,
    pub default_category: Option<Uuid>,
}

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
