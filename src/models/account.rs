use uuid::Uuid;
use bigdecimal::BigDecimal;
use schema::accounts;
use num_traits::Zero;
use rocket::http::RawStr;
use rocket::request::FromFormValue;

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
    pub balance: Balance,
    pub on_budget: bool,
}

pub struct Balance(pub BigDecimal);

impl<'v> FromFormValue<'v> for Balance {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Balance, &'v RawStr> {
        match BigDecimal::parse_bytes(&form_value.as_bytes(), 10) {
            Some(val) => Ok(Balance(val)),
            _ => Ok(Balance(BigDecimal::zero())),
        }
    }
}
