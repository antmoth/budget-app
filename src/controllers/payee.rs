use diesel::pg::PgConnection;
use diesel::{self, LoadDsl};

use models::category::*;
use models::payee::*;

pub fn create_payee<'a>(conn: &PgConnection, name: &'a str, default_category: Option<Category>) -> Payee {
    use schema::payees;

    let new_payee = NewPayee {
        name: name,
        default_category: match default_category {
            Some(cat) => Some(cat.id),
            _ => None,
        }
    };

    diesel::insert(&new_payee).into(payees::table)
        .get_result(conn)
        .expect("Error saving new payee")
}

