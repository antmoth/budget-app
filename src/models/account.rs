use uuid::Uuid;
use bigdecimal::BigDecimal;
use num_traits::identities::Zero;
use diesel::pg::PgConnection;
use diesel::{self, BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl};
use chrono::{DateTime, Local, NaiveDate, Utc};

use schema::accounts;
use models::form_values::*;
use models::transaction::{NewTransaction, Transaction};
use error::Error;

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

pub fn get_account(conn: &PgConnection, aid: Uuid) -> Result<Vec<(Account, Vec<Transaction>)>, Error> {
    use schema::accounts;

    let account = accounts::table
        .find(aid)
        .load(conn)?;
    let transactions = Transaction::belonging_to(&account)
        .load::<Transaction>(conn)?
        .grouped_by(&account);
    Ok(account.into_iter().zip(transactions).collect::<Vec<_>>())
}

pub fn get_accounts(conn: &PgConnection) -> Result<Vec<(Account, Vec<Transaction>)>, Error> {
    use schema::accounts;

    let accounts = accounts::table
        .load::<Account>(conn)?;
    let transactions = Transaction::belonging_to(&accounts)
        .load::<Transaction>(conn)?
        .grouped_by(&accounts);
    Ok(accounts.into_iter().zip(transactions).collect::<Vec<_>>())
}

pub fn create_account<'a>(conn: &PgConnection, account: &FormAccount) -> Result<(Account, Vec<Transaction>), Error> {
    use schema::{accounts, transactions};

    let new_account = NewAccount {
        name: &account.name,
        cleared_balance: account.balance.0.clone(),
        uncleared_balance: BigDecimal::zero(),
        on_budget: account.on_budget,
    };

    let created_account: Account = diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)?;

    let today = NaiveDate::parse_from_str(&format!("{}", Local::today()), "%F%:z")?;
    let new_transaction = NewTransaction {
        date: today,
        account_id: created_account.id,
        category_id: None,
        payee_id: None,
        parent_transaction_id: None,
        amount: created_account.cleared_balance.clone(),
        memo: Some("Initial balance"),
        cleared: true,
    };

    let created_transaction = diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .get_result(conn)?;

    Ok((created_account, vec![created_transaction]))
}

pub fn update_account<'a>(conn: &PgConnection, aid: Uuid, account: &FormAccount) -> Result<Account, Error> {
    use schema::accounts::{self, columns};

    let (ref _old_account, ref transactions) = get_account(conn, aid)?[0];

    let cleared = transactions.iter().filter(|t| t.cleared)
        .fold(BigDecimal::zero(), |acc, ref t| acc + t.amount.clone());
    let uncleared = transactions.iter().filter(|t| !t.cleared)
        .fold(BigDecimal::zero(), |acc, ref t| acc + t.amount.clone());

    Ok(diesel::update(accounts::table)
        .set((columns::name.eq(&account.name),
            columns::cleared_balance.eq(cleared),
            columns::uncleared_balance.eq(uncleared),
            columns::on_budget.eq(account.on_budget)))
        .get_result(conn)?)
}
