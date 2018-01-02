use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::pg::PgConnection;
use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl};

use schema::categories::{self, dsl};
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

pub fn get_category(conn: &PgConnection, cid: Uuid) -> Category {
    categories::table
        .find(cid)
        .first(conn)
        .expect(&format!("Unable to find category {}", cid))
}

pub fn get_all_categories(conn: &PgConnection) -> Vec<Category> {
    categories::table
        .load::<Category>(conn)
        .expect("Error loading categories")
}

pub fn get_categories(conn: &PgConnection, fluid: bool) -> Vec<Category> {
    categories::table
        .filter(dsl::fluid.eq(fluid))
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
