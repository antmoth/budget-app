use rocket_contrib::Template;
use context::Context;

#[get("/")]
pub fn index(context: Context) -> Template {
    Template::render("index", context)
}

#[get("/budget")]
pub fn budget(context: Context) -> Template {
    Template::render("budget", context)
}

