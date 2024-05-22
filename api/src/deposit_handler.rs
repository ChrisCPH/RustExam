use shared::response_models::{Response, ResponseBody};
use application::crud::{create, read, update, delete};
use domain::models::NewDeposit;
use rocket::{get, post, put};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/deposits")]
pub fn list_deposits() -> String {
    let deposits = read::list_deposits();

    let response = Response { body: ResponseBody::Deposits(deposits) };

    serde_json::to_string(&response).unwrap()
}

#[get("/deposit/<deposit_id>")]
pub fn list_deposit(deposit_id: i32) -> Result<String, NotFound<String>> {
    let deposit = read::list_deposit(deposit_id)?;

    let response = Response { body: ResponseBody::Deposit(deposit) };

    Ok(serde_json::to_string(&response).unwrap())
}


#[put("/update_deposit/<deposit_id>", format = "application/json", data = "<deposit>")]
pub fn update_deposit(deposit_id: i32, deposit: Json<NewDeposit>) -> Created<String> {
    update::update_deposit(deposit_id, &deposit.account_id, &deposit.amount, &deposit.deposit_date, &deposit.comment)
}

#[post("/new_deposit", format = "application/json", data = "<deposit>")]
pub fn create_deposit(deposit: Json<NewDeposit>) -> Created<String> {
    create::create_deposit(deposit)
}

#[get("/delete_deposit/<deposit_id>")]
pub fn delete_deposit(deposit_id: i32) -> Result<String, NotFound<String>> {
    let deposits = delete::delete_deposit(deposit_id)?;

    let response = Response { body: ResponseBody::Deposits(deposits) };

    Ok(serde_json::to_string(&response).unwrap())
}