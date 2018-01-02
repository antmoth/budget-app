use diesel::pg::PgConnection;
use uuid::Uuid;
use diesel::{self, QueryDsl, RunQueryDsl};

use schema::payees;
use models::form_values::FormUuid;
use models::category::Category;

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

pub fn get_payees(conn: &PgConnection) -> Vec<(Payee, Option<Category>)> {
    use schema::payees;
    use schema::categories;

    payees::table.left_join(categories::table)
        .load::<(Payee, Option<Category>)>(conn)
        .expect("Error loading payees")
}

pub fn create_payee<'a>(conn: &PgConnection, payee: &FormPayee) -> Payee {
    use schema::payees;

    let new_payee = NewPayee {
        name: &payee.name,
        default_category: match payee.default_category {
            Some(ref u) => Some(u.0),
            _ => None,
        }
    };

    diesel::insert_into(payees::table)
        .values(&new_payee)
        .get_result(conn)
        .expect("Error saving new payee")
}

