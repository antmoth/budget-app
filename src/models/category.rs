use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use schema::categories;
use models::form_values::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub allocated: Option<BigDecimal>,
    pub parent_category: Option<Uuid>,
    pub due_amount: Option<BigDecimal>,
    pub due_date: Option<NaiveDate>,
    pub recurring: bool,
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub allocated: Option<BigDecimal>,
    pub parent_category: Option<Uuid>,
    pub due_amount: Option<BigDecimal>,
    pub due_date: Option<NaiveDate>,
    pub recurring: bool,
}

#[derive(FromForm)]
pub struct FormCategory {
    pub name: String,
    pub allocated: Option<FormDecimal>,
    pub parent_category: Option<FormUuid>,
    pub due_amount: Option<FormDecimal>,
    pub due_date: Option<FormDate>,
    pub recurring: bool,
}
