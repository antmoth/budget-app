table! {
    accounts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Text,
    }
}

table! {
    categories (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Text,
        allocation -> Numeric,
        goal_amount -> Nullable<Numeric>,
        due_date -> Nullable<Date>,
    }
}

table! {
    transactions (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        date -> Date,
        account_id -> Uuid,
        amount -> Numeric,
        memo -> Nullable<Text>,
    }
}

joinable!(transactions -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    transactions,
);
