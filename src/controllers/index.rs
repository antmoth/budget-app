use rocket_contrib::Template;
use context::Context;
use bigdecimal::BigDecimal;
use num_traits::Zero;

use models::account;
use error::Error;

#[get("/")]
pub fn index(context: Context) -> Template {
    Template::render("index", context)
}

#[get("/budget")]
pub fn budget(mut context: Context) -> Result<Template, Error> {
    use models::category;

    let categories = category::get_categories(&context.db);
    let accounts = account::get_accounts(&context.db)?;
    let balances = accounts.iter()
        .map(|a| a.1.iter().map(|t| t.amount).fold(BigDecimal::zero(), |t, u| t + u))
        .fold(BigDecimal::zero(), |acc, x| acc + x);
    let allocated = categories.iter()
        .map(|c| c.allocation)
        .fold(BigDecimal::zero(), |acc, x| acc + x);
    let unallocated = balances.clone() - allocated.clone();

    context.data = json!({ "allocated": &allocated,
        "unallocated": &unallocated,
        "total": &balances,
        "categories": &categories,
        "accounts": &accounts
    });
    Ok(Template::render("budget", context))
}

