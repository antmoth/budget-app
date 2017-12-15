use uuid::Uuid;
use schema::payees;

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

