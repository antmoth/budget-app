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

    let fluid = category::get_categories(&context.db, false);
    let recurring = category::get_categories(&context.db, true);
    let accounts = account::get_accounts(&context.db)?;
    let accounts: Vec<&(Account, Vec<Transaction>)> = accounts.iter().filter(|a| a.0.on_budget).collect();
    let cleared = accounts.iter()
        .map(|a| a.0.cleared_balance.clone())
        .fold(BigDecimal::zero(), |acc, x| acc + x);
    let uncleared = accounts.iter()
        .map(|a| a.0.uncleared_balance.clone())
        .fold(BigDecimal::zero(), |acc, x| acc + x);

    context.data = json!({ "cleared": &cleared,
        "uncleared": &uncleared,
        "fluid": &fluid,
        "recurring": &recurring
    });
    Ok(Template::render("budget", context))
}

