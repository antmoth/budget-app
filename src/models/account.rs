use uuid::Uuid;
use bigdecimal::BigDecimal;
use num_traits::identities::Zero;
use diesel::pg::PgConnection;
use diesel::{self, RunQueryDsl};

use schema::accounts;
use models::form_values::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub cleared_balance: BigDecimal,
    pub uncleared_balance: BigDecimal,
    pub on_budget: bool,
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub cleared_balance: BigDecimal,
    pub uncleared_balance: BigDecimal,
    pub on_budget: bool,
}

#[derive(FromForm)]
pub struct FormAccount {
    pub name: String,
    pub balance: FormDecimal,
    pub on_budget: bool,
}

pub fn get_accounts(conn: &PgConnection) -> Vec<Account> {
    use schema::accounts::dsl::*;

    accounts
        .load::<Account>(conn)
        .expect("Error loading accounts")
}

pub fn create_account<'a>(conn: &PgConnection, account: &FormAccount) -> Account {
    use schema::accounts;

    let new_account = NewAccount {
        name: &account.name,
        cleared_balance: account.balance.0.clone(),
        uncleared_balance: BigDecimal::zero(),
        on_budget: account.on_budget,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Error saving new account")
}
