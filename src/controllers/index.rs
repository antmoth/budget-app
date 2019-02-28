use crate::context::Context;
use bigdecimal::BigDecimal;
use num_traits::Zero;
use rocket_contrib::templates::Template;

use crate::error::Error;
use crate::models::account;
use crate::MainDbConn;

#[get("/")]
pub fn index(context: Context) -> Template {
    Template::render("index", context)
}

#[get("/budget")]
pub fn budget(conn: MainDbConn, mut context: Context) -> Result<Template, Error> {
    use crate::models::category;

    let categories = category::get_categories(&conn);
    let accounts = account::get_accounts(&conn)?;
    let balances = accounts
        .iter()
        .map(|a| {
            a.1.iter()
                .map(|t| t.amount.clone())
                .fold(BigDecimal::zero(), |t, u| t + u)
        })
        .fold(BigDecimal::zero(), |acc, x| acc + x);
    let allocated = categories
        .iter()
        .map(|c| c.allocation.clone())
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
