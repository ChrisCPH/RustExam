use shared::response_models::{Response, ResponseBody};
use application::crud::{create, read, update, delete};
use domain::models::NewWithdraw;
use rocket::{get, post, put};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/withdraws")]
pub fn list_withdraws() -> String {
    let withdraws = read::list_withdraws();

    let response = Response { body: ResponseBody::Withdraws(withdraws) };

    serde_json::to_string(&response).unwrap()
}

#[get("/withdraw/<withdraw_id>")]
pub fn list_withdraw(withdraw_id: i32) -> Result<String, NotFound<String>> {
    let withdraw = read::list_withdraw(withdraw_id)?;

    let response = Response { body: ResponseBody::Withdraw(withdraw) };

    Ok(serde_json::to_string(&response).unwrap())
}


#[put("/update_withdraw/<withdraw_id>", format = "application/json", data = "<withdraw>")]
pub fn update_withdraw(withdraw_id: i32, withdraw: Json<NewWithdraw>) -> Created<String> {
    update::update_withdraw(withdraw_id, &withdraw.account_id, &withdraw.amount, &withdraw.withdraw_date, &withdraw.comment)
}

#[post("/new_withdraw", format = "application/json", data = "<withdraw>")]
pub fn create_withdraw(withdraw: Json<NewWithdraw>) -> Created<String> {
    create::create_withdraw(withdraw)
}

#[get("/delete_withdraw/<withdraw_id>")]
pub fn delete_withdraw(withdraw_id: i32) -> Result<String, NotFound<String>> {
    let withdraws = delete::delete_withdraw(withdraw_id)?;

    let response = Response { body: ResponseBody::Withdraws(withdraws) };

    Ok(serde_json::to_string(&response).unwrap())
}