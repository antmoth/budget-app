use rocket::Rocket;

mod index;
mod account;
mod category;
mod payee;
mod transaction;

pub fn mount(r: Rocket) -> Rocket {
    r.mount("/", [
        routes![index::index,
            index::budget,
            account::accounts,
            account::new_account,
            account::new_account_post,
            account::edit_account,
            account::edit_account_post,
            transaction::transactions,
            transaction::new_transaction,
            transaction::new_transaction_post,
            payee::payees,
            payee::new_payee,
            payee::new_payee_post,
            category::fluid_categories,
            category::goal_categories,
            category::new_category,
            category::new_category_post,
            category::edit_category]
    ].concat())
}
