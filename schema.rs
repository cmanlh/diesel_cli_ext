table! {
    carry_over_balances (account_id) {
        account_id -> BigInt,
        debit -> Numeric,
        description -> Nullable<Text>,
    }
}

table! {
    orders (id1, id2) {
        id1 -> BigInt,
        time -> Timestamp,
        time2 -> Timestamptz,
        json -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    carry_over_balances,
    orders,
);