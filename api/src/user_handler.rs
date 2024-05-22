use shared::response_models::{Response, ResponseBody};
use application::crud::{create, read, update, delete};
use domain::models::NewUser;
use rocket::{get, post, put};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/users")]
pub fn list_users() -> String {
    let users = read::list_users();

    let response = Response { body: ResponseBody::Users(users) };

    serde_json::to_string(&response).unwrap()
}

#[get("/user/<user_id>")]
pub fn list_user(user_id: i32) -> Result<String, NotFound<String>> {
    let user = read::list_user(user_id)?;

    let response = Response { body: ResponseBody::User(user) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/update_user/<user_id>", format = "application/json", data = "<user>")]
pub fn update_user(user_id: i32, user: Json<NewUser>) -> Created<String> {
    update::update_user(user_id, &user.user_name, &user.password)
}

#[post("/new_user", format = "application/json", data = "<user>")]
pub fn create_user(user: Json<NewUser>) -> Created<String> {
    create::create_user(user)
}

#[get("/delete_user/<user_id>")]
pub fn delete_user(user_id: i32) -> Result<String, NotFound<String>> {
    let users = delete::delete_user(user_id)?;

    let response = Response { body: ResponseBody::Users(users) };

    Ok(serde_json::to_string(&response).unwrap())
}
