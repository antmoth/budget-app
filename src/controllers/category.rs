use rocket::response::Redirect;
use rocket::request::LenientForm;
use rocket_contrib::templates::Template;

use MainDbConn;
use diesel::Connection;
use models::category::*;
use models::form_values::FormUuid;
use error::Error;
use context::Context;

#[get("/categories")]
pub fn categories(conn: MainDbConn, mut context: Context) -> Result<Template, Error> {
    let categories = get_categories(&conn);

    context.data = json!({ "categories": &categories,
        "fluid": true});
    Ok(Template::render("categories", context))
}

#[get("/new_category")]
pub fn new_category(conn: MainDbConn, context: Context) -> Template {
    Template::render("edit_category", context)
}

#[post("/new_category", data="<category>")]
pub fn new_category_post(conn: MainDbConn, context: Context, category: LenientForm<FormCategory>) -> Result<Redirect, Error> {
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
