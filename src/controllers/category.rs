use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::{self, RunQueryDsl};
use rocket::response::Redirect;
use rocket::request::LenientForm;

use models::category::*;
use error::Error;
use context::Context;

#[post("/new_category", data="<category>")]
pub fn new_category_post(context: Context, category: LenientForm<FormCategory>) -> Result<Redirect, Error> {
    let category = category.get();

    context.db.transaction(|| {
        create_category(&context.db, &category);
        Ok(Redirect::to("/budget"))
    })
    .or_else(|e| Err(e))
}

pub fn create_category<'a>(conn: &PgConnection, category: &FormCategory) -> Category {
    use schema::categories;

    let new_category = NewCategory {
        name: &category.name,
        allocated: match category.allocated {
            Some(ref a) => Some(a.0.clone()),
            _ => None,
        },
        parent_category: match category.parent_category {
            Some(ref u) => Some(u.0),
            _ => None,
        },
        due_amount: match category.due_amount {
            Some(ref a) => Some(a.0.clone()),
            _ => None,
        },
        due_date: match category.due_date {
            Some(ref d) => Some(d.0),
            _ => None,
        },
    };

    diesel::insert_into(categories::table)
        .values(&new_category)
        .get_result(conn)
        .expect("Error saving new  category")
}

