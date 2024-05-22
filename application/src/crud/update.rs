use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;

use domain::models::Users;
use domain::schema::users::dsl::*;

use domain::models::Accounts;
use domain::schema::accounts::dsl::*;

use domain::models::Deposit;
use domain::schema::deposit::dsl::*;

use domain::models::Withdraw;
use domain::schema::withdraw::dsl::*;

use domain::models::UserAccount;
use domain::schema::user_account::dsl::*;

// User

pub fn update_user(id: i32, name: &str, pw: &str) -> Created<String> {

    match diesel::update(users.filter(domain::schema::users::user_id.eq(id)))
    .set((user_name.eq(name), password.eq(pw)))
    .get_result::<Users>(&mut establish_connection()) {
        Ok(user) => {
            let response = Response { body: ResponseBody::User(user) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

// Account

pub fn update_account(id: i32, balance_: &i32, currency_: &str, account_type_: &str) -> Created<String> {

    match diesel::update(accounts.filter(domain::schema::accounts::account_id.eq(id)))
    .set((balance.eq(balance_), currency.eq(currency_), account_type.eq(account_type_)))
    .get_result::<Accounts>(&mut establish_connection()) {
        Ok(account) => {
            let response = Response { body: ResponseBody::Account(account) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

// Deposit

pub fn update_deposit(id: i32, account_id_: &i32, amount_: &i32, deposit_date_: &str, comment_: &str) -> Created<String> {

    match diesel::update(deposit.filter(deposit_id.eq(id)))
    .set((domain::schema::deposit::account_id.eq(account_id_), domain::schema::deposit::amount.eq(amount_), deposit_date.eq(deposit_date_), domain::schema::deposit::comment.eq(comment_)))
    .get_result::<Deposit>(&mut establish_connection()) {
        Ok(deposits) => {
            let response = Response { body: ResponseBody::Deposit(deposits) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

// Withdraw

pub fn update_withdraw(id: i32, account_id_: &i32, amount_: &i32, withdraw_date_: &str, comment_: &str) -> Created<String> {

    match diesel::update(withdraw.filter(withdraw_id.eq(id)))
    .set((domain::schema::withdraw::account_id.eq(account_id_), domain::schema::withdraw::amount.eq(amount_), withdraw_date.eq(withdraw_date_), domain::schema::withdraw::comment.eq(comment_)))
    .get_result::<Withdraw>(&mut establish_connection()) {
        Ok(withdraws) => {
            let response = Response { body: ResponseBody::Withdraw(withdraws) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

// UserAccount

pub fn update_user_account(id: i32, user_id_: &i32 , account_id_: &i32) -> Created<String> {

    match diesel::update(user_account.filter(user_account_id.eq(id)))
    .set((domain::schema::user_account::account_id.eq(account_id_), domain::schema::user_account::account_id.eq(user_id_)))
    .get_result::<UserAccount>(&mut establish_connection()) {
        Ok(user_accounts) => {
            let response = Response { body: ResponseBody::UserAccount(user_accounts) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}