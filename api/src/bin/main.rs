#[macro_use] extern crate rocket;
use api::user_handler;
use api::account_handler;
use api::deposit_handler;
use api::withdraw_handler;
use api::user_account_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            user_handler::list_users,
            user_handler::list_user,
            user_handler::create_user,
            user_handler::update_user,
            user_handler::delete_user,

            account_handler::list_accounts,
            account_handler::list_account,
            account_handler::create_account,
            account_handler::update_account,
            account_handler::delete_account,

            deposit_handler::list_deposits,
            deposit_handler::list_deposit,
            deposit_handler::create_deposit,
            deposit_handler::update_deposit,
            deposit_handler::delete_deposit,

            withdraw_handler::list_withdraws,
            withdraw_handler::list_withdraw,
            withdraw_handler::create_withdraw,
            withdraw_handler::update_withdraw,
            withdraw_handler::delete_withdraw,

            user_account_handler::list_user_accounts,
            user_account_handler::list_user_account,
            user_account_handler::create_user_account,
            user_account_handler::update_user_account,
            user_account_handler::delete_user_account,
            
        ])
}

