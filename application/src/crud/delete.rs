use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

use domain::models::Users;
use domain::schema::users::dsl::*;
use domain::schema::users;

use domain::models::Accounts;
use domain::schema::accounts::dsl::*;
use domain::schema::accounts;

use domain::models::Deposit;
use domain::schema::deposit::dsl::*;
use domain::schema::deposit;

use domain::models::Withdraw;
use domain::schema::withdraw::dsl::*;
use domain::schema::withdraw;

use domain::models::UserAccount;
use domain::schema::user_account::dsl::*;
use domain::schema::user_account;

// User

pub fn delete_user(id: i32) -> Result<Vec<Users>, NotFound<String>> {

    let response: Response;

    let num_deleted = match diesel::delete(users.filter(domain::schema::users::user_id.eq(id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing user with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match users::table.select(users::all_columns).load::<Users>(&mut establish_connection()) {
            Ok(mut users_) => {
                users_.sort();
                Ok(users_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no user with id {}", id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}

// Account

pub fn delete_account(id: i32) -> Result<Vec<Accounts>, NotFound<String>> {

    let response: Response;

    let num_deleted = match diesel::delete(accounts.filter(domain::schema::accounts::account_id.eq(id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing account with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match accounts::table.select(accounts::all_columns).load::<Accounts>(&mut establish_connection()) {
            Ok(mut accounts_) => {
                accounts_.sort();
                Ok(accounts_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no account with id {}", id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}

// Deposit

pub fn delete_deposit(id: i32) -> Result<Vec<Deposit>, NotFound<String>> {

    let response: Response;

    let num_deleted = match diesel::delete(deposit.filter(deposit_id.eq(id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing deposit with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match deposit::table.select(deposit::all_columns).load::<Deposit>(&mut establish_connection()) {
            Ok(mut deposit_) => {
                deposit_.sort();
                Ok(deposit_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no deposit with id {}", id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}

// Withdraw

pub fn delete_withdraw(id: i32) -> Result<Vec<Withdraw>, NotFound<String>> {

    let response: Response;

    let num_deleted = match diesel::delete(withdraw.filter(withdraw_id.eq(id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing withdraw with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match withdraw::table.select(withdraw::all_columns).load::<Withdraw>(&mut establish_connection()) {
            Ok(mut withdraw_) => {
                withdraw_.sort();
                Ok(withdraw_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no withdraw with id {}", id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}

// UserAccount

pub fn delete_user_account(id: i32) -> Result<Vec<UserAccount>, NotFound<String>> {

    let response: Response;

    let num_deleted = match diesel::delete(user_account.filter(user_account_id.eq(id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing user_account with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match user_account::table.select(user_account::all_columns).load::<UserAccount>(&mut establish_connection()) {
            Ok(mut user_account_) => {
                user_account_.sort();
                Ok(user_account_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no user_account with id {}", id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}