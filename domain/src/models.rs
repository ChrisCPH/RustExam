use crate::schema::users;
use crate::schema::accounts;
use crate::schema::deposit;
use crate::schema::withdraw;
use crate::schema::user_account;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Serialize, Queryable, Ord, Eq, PartialOrd, PartialEq)]
pub struct Users {
    pub user_id: i32,
    pub user_name: String,
    pub password: String,
}

#[derive(Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Queryable, Ord, Eq, PartialOrd, PartialEq)]
pub struct Accounts {
    pub account_id: i32,
    pub balance: i32,
    pub currency: String,
    pub account_type: String,
}

#[derive(Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub balance: i32,
    pub currency: String,
    pub account_type: String,
}

#[derive(Serialize, Queryable, Ord, Eq, PartialOrd, PartialEq)]
pub struct Deposit {
    pub deposit_id: i32,
    pub account_id: i32,
    pub amount: i32,
    pub deposit_date: String, 
    pub comment: String,
}

#[derive(Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = deposit)]
pub struct NewDeposit {
    pub account_id: i32,
    pub amount: i32,
    pub deposit_date: String,
    pub comment: String,
}

#[derive(Serialize, Queryable, Ord, Eq, PartialOrd, PartialEq)]
pub struct Withdraw {
    pub withdraw_id: i32,
    pub account_id: i32,
    pub amount: i32,
    pub withdraw_date: String,
    pub comment: String,
}

#[derive(Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = withdraw)]
pub struct NewWithdraw {
    pub account_id: i32,
    pub amount: i32,
    pub withdraw_date: String,
    pub comment: String,
}

#[derive(Serialize, Queryable, Ord, Eq, PartialOrd, PartialEq)]
pub struct UserAccount {
    pub user_account_id: i32,
    pub user_id: i32,
    pub account_id: i32,
}

#[derive(Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = user_account)]
pub struct NewUserAccount {
    pub user_id: i32,
    pub account_id: i32,
}
