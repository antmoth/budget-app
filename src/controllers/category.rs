use diesel::pg::PgConnection;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{self, LoadDsl};

use models::category::*;

pub fn create_category<'a>(conn: &PgConnection, name: &'a str, allocated: Option<BigDecimal>, parent_category: Option<Category>, due_amount: Option<BigDecimal>, due_date: Option<NaiveDate>) -> Category {
    use schema::categories;

    let new_category = NewCategory {
        name: name,
        allocated: allocated,
        parent_category: match parent_category {
            Some(cat) => Some(cat.id),
            _ => None,
        },
        due_amount: due_amount,
        due_date: due_date,
    };

    diesel::insert(&new_category).into(categories::table)
        .get_result(conn)
        .expect("Error saving new  category")
}

