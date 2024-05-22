use domain::models::Users;
use domain::models::Accounts;
use domain::models::Withdraw;
use domain::models::Deposit;
use domain::models::UserAccount;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    User(Users),
    Users(Vec<Users>),
    Account(Accounts),
    Accounts(Vec<Accounts>),
    Withdraw(Withdraw),
    Withdraws(Vec<Withdraw>),
    Deposit(Deposit),
    Deposits(Vec<Deposit>),
    UserAccount(UserAccount),
    UserAccounts(Vec<UserAccount>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
