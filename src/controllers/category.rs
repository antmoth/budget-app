use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::{self, RunQueryDsl};
use rocket::response::Redirect;
use rocket::request::LenientForm;
use rocket_contrib::Template;

use models::category::*;
use error::Error;
use context::Context;

#[get("/categories")]
pub fn categories(mut context: Context) -> Result<Template, Error> {
    let categories = get_categories(&context.db);

    context.data = json!({ "categories": &categories });
    Ok(Template::render("categories", context))
}

#[get("/new_category")]
pub fn new_category(context: Context) -> Template {
    Template::render("edit_category", context)
}

#[post("/new_category", data="<category>")]
pub fn new_category_post(context: Context, category: LenientForm<FormCategory>) -> Result<Redirect, Error> {
    let category = category.get();

    context.db.transaction(|| {
        create_category(&context.db, &category);
        Ok(Redirect::to("/budget"))
    })
    .or_else(|e| Err(e))
}

fn get_categories(conn: &PgConnection) -> Vec<Category> {
    use schema::categories::dsl::*;

    categories
        .load::<Category>(conn)
        .expect("Error loading categories")
}

fn create_category<'a>(conn: &PgConnection, category: &FormCategory) -> Category {
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

