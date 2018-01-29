use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Duration, Local, NaiveDate, Utc};
use diesel::pg::PgConnection;
use diesel::{self, QueryDsl, RunQueryDsl};

use schema::categories;
use models::form_values::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub allocated: Option<BigDecimal>,
    pub parent_category_id: Option<Uuid>,
    pub due_amount: Option<BigDecimal>,
    pub due_date: Option<NaiveDate>,
    pub fluid: bool,
}

impl Category {
    fn time_left(&self) -> Duration {
        let today = NaiveDate::parse_from_str(&format!("{}", Local::today()), "%F%:z").expect("Could not parse today's date");
        let due = match self.due_date {
            Some(d) => d,
            None => today,
        };
        return today.signed_duration_since(due)
    }
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub allocated: Option<BigDecimal>,
    pub parent_category_id: Option<Uuid>,
    pub due_amount: Option<BigDecimal>,
    pub due_date: Option<NaiveDate>,
    pub fluid: bool,
}

#[derive(FromForm)]
pub struct FormCategory {
    pub name: String,
    pub allocated: Option<FormDecimal>,
    pub parent_category_id: Option<FormUuid>,
    pub due_amount: Option<FormDecimal>,
    pub due_date: Option<FormDate>,
    pub fluid: bool,
}

pub struct WebCategory {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub allocated: Option<BigDecimal>,
    pub parent_category_id: Option<Uuid>,
    pub due_amount: Option<BigDecimal>,
    pub due_date: Option<NaiveDate>,
    pub fluid: bool,
    pub spent_recently: Option<BigDecimal>,
    pub time_left: Option<Duration>,
    pub due_left: Option<BigDecimal>,
}

pub fn get_category(conn: &PgConnection, cid: Uuid) -> Category {
    categories::table
        .find(cid)
        .first(conn)
        .expect(&format!("Unable to find category {}", cid))
}

pub fn get_categories(conn: &PgConnection) -> Vec<Category> {
    categories::table
        .load::<Category>(conn)
        .expect("Error loading categories")
}

pub fn create_category<'a>(conn: &PgConnection, category: &FormCategory) -> Category {
    use schema::categories;

    let new_category = NewCategory {
        name: &category.name,
        allocated: match category.allocated {
            Some(ref a) => Some(a.0.clone()),
            _ => None,
        },
        parent_category_id: match category.parent_category_id {
            Some(ref cid) => Some(cid.0),
            _ => None,
        },
        due_amount: match category.due_amount {
            Some(ref a) => Some(a.0.clone()),
            _ => None,
        },
        due_date: match category.due_date {
            Some(ref d) => Some(d.0),
            _ => None,
        },
        fluid: category.fluid,
    };

    diesel::insert_into(categories::table)
        .values(&new_category)
        .get_result(conn)
        .expect("Error saving new  category")
}
