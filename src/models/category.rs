use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use schema::categories;

#[derive(Queryable)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub allocated: Option<BigDecimal>,
    pub parent_category: Option<Uuid>,
    pub due_amount: Option<BigDecimal>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub allocated: Option<BigDecimal>,
    pub parent_category: Option<Uuid>,
    pub due_amount: Option<BigDecimal>,
    pub due_date: Option<NaiveDate>,
}

