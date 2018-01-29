use rocket_contrib::Template;
use context::Context;
use bigdecimal::BigDecimal;
use num_traits::Zero;

use models::account::{self, Account};
use models::transaction::Transaction;
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
    let budget_accounts: Vec<&(Account, Vec<Transaction>)> = accounts.iter().filter(|a| a.0.on_budget).collect();
    let cleared = budget_accounts.iter()
        .map(|a| a.0.cleared_balance.clone())
        .fold(BigDecimal::zero(), |acc, x| acc + x);
    let allocated = categories.iter()
        .map(|c| match c.allocated.clone() {
            Some(a) => a,
            None => BigDecimal::zero()
        })
        .fold(BigDecimal::zero(), |acc, x| acc + x);
    let unallocated = cleared.clone() - allocated.clone();

    context.data = json!({ "allocated": &allocated,
        "unallocated": &unallocated,
        "total": &cleared,
        "categories": &categories,
        "accounts": &accounts
    });
    Ok(Template::render("budget", context))
}

