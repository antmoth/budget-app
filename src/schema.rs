table! {
    accounts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Text,
        cleared_balance -> Numeric,
        uncleared_balance -> Numeric,
        on_budget -> Bool,
    }
}

table! {
    categories (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Text,
        allocated -> Nullable<Numeric>,
        parent_category_id -> Nullable<Uuid>,
        due_amount -> Nullable<Numeric>,
        due_date -> Nullable<Date>,
        fluid -> Bool,
    }
}

table! {
    payees (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Text,
        default_category_id -> Nullable<Uuid>,
    }
}

table! {
    transactions (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        date -> Date,
        account_id -> Uuid,
        category_id -> Nullable<Uuid>,
        payee_id -> Nullable<Uuid>,
        parent_transaction_id -> Nullable<Uuid>,
        amount -> Numeric,
        memo -> Nullable<Text>,
        cleared -> Bool,
    }
}

joinable!(payees -> categories (default_category_id));
joinable!(transactions -> accounts (account_id));
joinable!(transactions -> categories (category_id));
joinable!(transactions -> payees (payee_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    payees,
    transactions,
);
