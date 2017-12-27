use uuid::Uuid;
use schema::payees;
use models::form_values::FormUuid;

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(FromForm)]
pub struct FormPayee {
    pub name: String,
    pub default_category: Option<FormUuid>,
}
