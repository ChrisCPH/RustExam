use shared::response_models::{Response, ResponseBody};
use application::crud::{create, read, update, delete};
use domain::models::NewUserAccount;
use rocket::{get, post, put};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/user_accounts")]
pub fn list_user_accounts() -> String {
    let user_accounts = read::list_user_accounts();

    let response = Response { body: ResponseBody::UserAccounts(user_accounts) };

    serde_json::to_string(&response).unwrap()
}

#[get("/user_account/<user_account_id>")]
pub fn list_user_account(user_account_id: i32) -> Result<String, NotFound<String>> {
    let user_account = read::list_user_account(user_account_id)?;

    let response = Response { body: ResponseBody::UserAccount(user_account) };

    Ok(serde_json::to_string(&response).unwrap())
}


#[put("/update_user_account/<user_account_id>", format = "application/json", data = "<user_account>")]
pub fn update_user_account(user_account_id: i32, user_account: Json<NewUserAccount>) -> Created<String> {
    update::update_user_account(user_account_id, &user_account.account_id, &user_account.user_id)
}

#[post("/new_user_account", format = "application/json", data = "<user_account>")]
pub fn create_user_account(user_account: Json<NewUserAccount>) -> Created<String> {
    create::create_user_account(user_account)
}

#[get("/delete_user_account/<user_account_id>")]
pub fn delete_user_account(user_account_id: i32) -> Result<String, NotFound<String>> {
    let user_accounts = delete::delete_user_account(user_account_id)?;

    let response = Response { body: ResponseBody::UserAccounts(user_accounts) };

    Ok(serde_json::to_string(&response).unwrap())
}