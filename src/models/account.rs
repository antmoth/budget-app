use uuid::Uuid;
use bigdecimal::BigDecimal;
use schema::accounts;

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

#[derive(FromForm)]
pub struct FormAccount {
    pub name: String,
    pub balance: usize,
    pub on_budget: bool,
}
