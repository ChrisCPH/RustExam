use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use domain::models::{Users, NewUser};
use domain::schema::users;

use domain::models::{Accounts, NewAccount};
use domain::schema::accounts;

use domain::models::{Deposit, NewDeposit};
use domain::schema::deposit;

use domain::models::{Withdraw, NewWithdraw};
use domain::schema::withdraw;

use domain::models::{UserAccount, NewUserAccount};
use domain::schema::user_account;

// User

pub fn create_user(user: Json<NewUser>) -> Created<String> {
    
    let user = user.into_inner();

    match diesel::insert_into(users::table)
    .values(&user)
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

pub fn create_account(account: Json<NewAccount>) -> Created<String> {
    
    let account = account.into_inner();

    match diesel::insert_into(accounts::table)
    .values(&account)
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

pub fn create_deposit(deposit: Json<NewDeposit>) -> Created<String> {
    let deposit = deposit.into_inner();

    let connection = &mut establish_connection();

    connection.transaction::<_, diesel::result::Error, _>(|conn| {

        let new_deposit = diesel::insert_into(deposit::table)
            .values(&deposit)
            .get_result::<Deposit>(conn)?;

        let mut account: Accounts = accounts::table
            .filter(accounts::account_id.eq(deposit.account_id))
            .first(conn)?;

        account.balance += deposit.amount;

        diesel::update(accounts::table.filter(accounts::account_id.eq(deposit.account_id)))
            .set(accounts::balance.eq(account.balance))
            .execute(conn)?;

        Ok(new_deposit)
    }).map(|deposit| {
        let response = Response { body: ResponseBody::Deposit(deposit) };
        Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
    }).map_err(|err| {
        panic!("Database error - {}", err);
    }).unwrap()
}

// Withdraw

pub fn create_withdraw(withdraw: Json<NewWithdraw>) -> Created<String> {
    let withdraw = withdraw.into_inner();

    let connection = &mut establish_connection();

    connection.transaction::<_, diesel::result::Error, _>(|conn| {

        let mut account: Accounts = accounts::table
            .filter(accounts::account_id.eq(withdraw.account_id))
            .first(conn)?;

        if account.balance < withdraw.amount {
            return Err(diesel::result::Error::RollbackTransaction);
        }

        let new_withdraw = diesel::insert_into(withdraw::table)
            .values(&withdraw)
            .get_result::<Withdraw>(conn)?;

        account.balance -= withdraw.amount;

        diesel::update(accounts::table.filter(accounts::account_id.eq(withdraw.account_id)))
            .set(accounts::balance.eq(account.balance))
            .execute(conn)?;

        Ok(new_withdraw)
    }).map(|withdraw| {
        let response = Response { body: ResponseBody::Withdraw(withdraw) };
        Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
    }).map_err(|err| {
        panic!("Database error - {}", err);
    }).unwrap()
}

// UserAcccount

pub fn create_user_account(user_account: Json<NewUserAccount>) -> Created<String> {
    
    let user_account = user_account.into_inner();

    match diesel::insert_into(user_account::table)
    .values(&user_account)
    .get_result::<UserAccount>(&mut establish_connection()) {
        Ok(user_account) => {
            let response = Response { body: ResponseBody::UserAccount(user_account) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}