use diesel::{Connection};
use rocket::response::Redirect;
use rocket::request::LenientForm;
use rocket_contrib::Template;

use models::category::*;
use models::form_values::FormUuid;
use error::Error;
use context::Context;

#[get("/goal_categories")]
pub fn goal_categories(mut context: Context) -> Result<Template, Error> {
    let categories = get_categories(&context.db, false);

    context.data = json!({ "categories": &categories,
        "fluid": false});
    Ok(Template::render("categories", context))
}

#[get("/fluid_categories")]
pub fn fluid_categories(mut context: Context) -> Result<Template, Error> {
    let categories = get_categories(&context.db, true);

    context.data = json!({ "categories": &categories,
        "fluid": true});
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

#[get("/edit_category/<id>")]
pub fn edit_category(mut context: Context, id: FormUuid) -> Template {
    let category = get_category(&context.db, id.0);

    context.data = json!({ "category": &category });
    Template::render("edit_category", context)
}
