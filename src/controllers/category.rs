use rocket::request::LenientForm;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::context::Context;
use crate::error::Error;
use crate::models::category::*;
use crate::models::form_values::FormUuid;
use crate::MainDbConn;
use diesel::Connection;

#[get("/categories")]
pub fn categories(conn: MainDbConn, mut context: Context) -> Result<Template, Error> {
    let categories = get_categories(&conn);

    context.data = json!({ "categories": &categories,
        "fluid": true});
    Ok(Template::render("categories", context))
}

#[get("/new_category")]
pub fn new_category(_conn: MainDbConn, context: Context) -> Template {
    Template::render("edit_category", context)
}

#[post("/new_category", data = "<category>")]
pub fn new_category_post(
    conn: MainDbConn,
    _context: Context,
    category: LenientForm<FormCategory>,
) -> Result<Redirect, Error> {
    let category = category.into_inner();

    conn.transaction(|| {
        create_category(&conn, &category);
        Ok(Redirect::to("/budget"))
    })
    .or_else(|e| Err(e))
}

#[get("/edit_category/<id>")]
pub fn edit_category(conn: MainDbConn, mut context: Context, id: FormUuid) -> Template {
    let category = get_category(&conn, id.0);

    context.data = json!({ "category": &category });
    Template::render("edit_category", context)
}
