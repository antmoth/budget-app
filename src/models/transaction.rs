use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use schema::transactions;
use models::account::Balance;
use rocket::http::RawStr;
use rocket::request::FromFormValue;

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
    pub amount: Balance,
    pub memo: Option<String>,
    pub cleared: bool,
}

pub struct FormDate(pub NaiveDate);

impl<'v> FromFormValue<'v> for FormDate {
    type Error = &'v str;

    fn from_form_value(form_value: &'v RawStr) -> Result<FormDate, &'v str> {
        match NaiveDate::parse_from_str(&form_value, "%Y-%m-%d") {
            Ok(date) => Ok(FormDate(date)),
            _ => Err("Unable to parse date")
        }
    }
}

pub struct FormUuid(pub Uuid);

impl<'v> FromFormValue<'v> for FormUuid {
    type Error = &'v str;

    fn from_form_value(form_value: &'v RawStr) -> Result<FormUuid, &'v str> {
        match Uuid::parse_str(&form_value) {
            Ok(uuid) => Ok(FormUuid(uuid)),
            _ => Err("Unable to parse uuid")
        }
    }
}
