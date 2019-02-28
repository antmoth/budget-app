use bigdecimal::BigDecimal;
use chrono::{DateTime, Local, NaiveDate, Utc};
use diesel::pg::PgConnection;
use diesel::{self, BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl};
use num_traits::identities::Zero;
use uuid::Uuid;

use crate::error::Error;
use crate::models::form_values::*;
use crate::models::transaction::{NewTransaction, Transaction};
use crate::schema::accounts;

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

pub struct NewAccount<'a> {
    pub name: &'a str,
    pub initial_balance: BigDecimal,
}

#[derive(FromForm)]
pub struct FormAccount {
    pub name: String,
    pub balance: Option<FormDecimal>,
}

pub fn get_account(
    conn: &PgConnection,
    aid: Uuid,
) -> Result<Vec<(Account, Vec<Transaction>)>, Error> {
    use crate::schema::accounts;

    let account = accounts::table.find(aid).load(conn)?;
    let transactions = Transaction::belonging_to(&account)
        .load::<Transaction>(conn)?
        .grouped_by(&account);
    Ok(account.into_iter().zip(transactions).collect::<Vec<_>>())
}

pub fn get_accounts(conn: &PgConnection) -> Result<Vec<(Account, Vec<Transaction>)>, Error> {
    use crate::schema::accounts;

    let accounts = accounts::table.load::<Account>(conn)?;
    let transactions = Transaction::belonging_to(&accounts)
        .load::<Transaction>(conn)?
        .grouped_by(&accounts);
    Ok(accounts.into_iter().zip(transactions).collect::<Vec<_>>())
}

pub fn create_account(
    conn: &PgConnection,
    account: &FormAccount,
) -> Result<(Account, Vec<Transaction>), Error> {
    use crate::schema::{accounts, transactions};

    let new_account = NewAccount {
        name: &account.name,
        initial_balance: match &account.balance {
            Some(ref form_val) => form_val.0.clone(),
            None => BigDecimal::zero(),
        },
    };

    let created_account: Account = diesel::insert_into(accounts::table)
        .values(accounts::columns::name.eq(&new_account.name))
        .get_result(conn)?;

    let today = NaiveDate::parse_from_str(&format!("{}", Local::today()), "%F%:z")?;
    let new_transaction = NewTransaction {
        date: today,
        account_id: created_account.id,
        amount: new_account.initial_balance.clone(),
        memo: Some("Initial balance"),
    };

    let created_transaction = diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .get_result(conn)?;

    Ok((created_account, vec![created_transaction]))
}

pub fn update_account(
    conn: &PgConnection,
    aid: Uuid,
    account: &FormAccount,
) -> Result<Account, Error> {
    use crate::schema::accounts::{self, columns};

    let (ref _old_account, ref _transactions) = get_account(conn, aid)?[0];

    Ok(diesel::update(accounts::table)
        .set(columns::name.eq(&account.name))
        .get_result(conn)?)
}
