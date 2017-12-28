use rocket_contrib::Template;
use context::Context;
use bigdecimal::BigDecimal;
use num_traits::Zero;

use models::account::Account;

#[get("/")]
pub fn index(context: Context) -> Template {
    Template::render("index", context)
}

#[get("/budget")]
pub fn budget(mut context: Context) -> Template {
    use controllers;

    let fluid = controllers::category::get_categories(&context.db, false);
    let recurring = controllers::category::get_categories(&context.db, true);
    let accounts = controllers::account::get_accounts(&context.db);
    let accounts: Vec<&Account> = accounts.iter().filter(|a| a.on_budget).collect();
    let cleared = accounts.iter()
        .map(|a| a.cleared_balance.clone())
        .fold(BigDecimal::zero(), |acc, x| acc + x);
    let uncleared = accounts.iter()
        .map(|a| a.uncleared_balance.clone())
        .fold(BigDecimal::zero(), |acc, x| acc + x);

    context.data = json!({ "cleared": &cleared,
        "uncleared": &uncleared,
        "fluid": &fluid,
        "recurring": &recurring
    });
    Template::render("budget", context)
}

