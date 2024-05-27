use shared::response_models::{Response, ResponseBody};
use application::crud::{create, read, update, delete};
use domain::models::NewAccount;
use rocket::{get, post, put};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/accounts")]
pub fn list_accounts() -> String {
    let accounts = read::list_accounts();

    let response = Response { body: ResponseBody::Accounts(accounts) };

    serde_json::to_string(&response).unwrap()
}

#[get("/account/<account_id>")]
pub fn list_account(account_id: i32) -> Result<String, NotFound<String>> {
    let account = read::list_account(account_id)?;

    let response = Response { body: ResponseBody::Account(account) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/update_account/<account_id>", format = "application/json", data = "<account>")]
pub fn update_account(account_id: i32, account: Json<NewAccount>) -> Created<String> {
    update::update_account(account_id, &account.balance, &account.currency, &account.account_type)
}

#[post("/new_account", format = "application/json", data = "<account>")]
pub fn create_account(account: Json<NewAccount>) -> Created<String> {
    create::create_account(account)
}

#[get("/delete_account/<account_id>")]
pub fn delete_account(account_id: i32) -> Result<String, NotFound<String>> {
    let accounts = delete::delete_account(account_id)?;

    let response = Response { body: ResponseBody::Accounts(accounts) };

    Ok(serde_json::to_string(&response).unwrap())
}