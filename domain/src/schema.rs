// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (account_id) {
        account_id -> Int4,
        balance -> Int4,
        currency -> Varchar,
        account_type -> Varchar,
    }
}

diesel::table! {
    deposit (deposit_id) {
        deposit_id -> Int4,
        account_id -> Int4,
        amount -> Int4,
        deposit_date -> Varchar,
        comment -> Varchar,
    }
}

diesel::table! {
    user_account (user_account_id) {
        user_account_id -> Int4,
        user_id -> Int4,
        account_id -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    withdraw (withdraw_id) {
        withdraw_id -> Int4,
        account_id -> Int4,
        amount -> Int4,
        withdraw_date -> Varchar,
        comment -> Varchar,
    }
}

diesel::joinable!(deposit -> accounts (account_id));
diesel::joinable!(user_account -> accounts (account_id));
diesel::joinable!(user_account -> users (user_id));
diesel::joinable!(withdraw -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    deposit,
    user_account,
    users,
    withdraw,
);
