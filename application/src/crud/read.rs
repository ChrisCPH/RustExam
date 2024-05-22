use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

use domain::models::Users;
use domain::schema::users;

use domain::models::Accounts;
use domain::schema::accounts;

use domain::models::Deposit;
use domain::schema::deposit;

use domain::models::Withdraw;
use domain::schema::withdraw;

use domain::models::UserAccount;
use domain::schema::user_account;

// Users
pub fn list_user(id: i32) -> Result<Users, NotFound<String>> {

    match users::table.find(id).first::<Users>(&mut establish_connection()) {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting user with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_users() -> Vec<Users> {

    match users::table.select(users::all_columns).load::<Users>(&mut establish_connection()) {
        Ok(mut users) => {
            users.sort();
            users
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}


// Accounts
pub fn list_account(id: i32) -> Result<Accounts, NotFound<String>> {

    match accounts::table.find(id).first::<Accounts>(&mut establish_connection()) {
        Ok(account) => Ok(account),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting account with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_accounts() -> Vec<Accounts> {

    match accounts::table.select(accounts::all_columns).load::<Accounts>(&mut establish_connection()) {
        Ok(mut accounts) => {
            accounts.sort();
            accounts
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

// Deposit

pub fn list_deposit(id: i32) -> Result<Deposit, NotFound<String>> {

    match deposit::table.find(id).first::<Deposit>(&mut establish_connection()) {
        Ok(deposit) => Ok(deposit),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting deposit with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_deposits() -> Vec<Deposit> {

    match deposit::table.select(deposit::all_columns).load::<Deposit>(&mut establish_connection()) {
        Ok(mut deposit) => {
            deposit.sort();
            deposit
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

// Withdraw

pub fn list_withdraw(id: i32) -> Result<Withdraw, NotFound<String>> {

    match withdraw::table.find(id).first::<Withdraw>(&mut establish_connection()) {
        Ok(withdraw) => Ok(withdraw),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting withdraw with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_withdraws() -> Vec<Withdraw> {

    match withdraw::table.select(withdraw::all_columns).load::<Withdraw>(&mut establish_connection()) {
        Ok(mut withdraw) => {
            withdraw.sort();
            withdraw
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

// UserAccount

pub fn list_user_account(id: i32) -> Result<UserAccount, NotFound<String>> {

    match user_account::table.find(id).first::<UserAccount>(&mut establish_connection()) {
        Ok(user_account) => Ok(user_account),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting user_account with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_user_accounts() -> Vec<UserAccount> {

    match user_account::table.select(user_account::all_columns).load::<UserAccount>(&mut establish_connection()) {
        Ok(mut user_account) => {
            user_account.sort();
            user_account
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}