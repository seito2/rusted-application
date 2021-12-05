use actix_web::{HttpRequest, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::entity::account::AccountEntity;
use crate::models::Account;
use crate::modules::errors::ApiErrorResponse;
use crate::util::db::establish_connection;

#[derive(Serialize, Deserialize)]
struct MyObj {
    str: String,
    num: isize,
    arr: Vec<isize>,
}

#[derive(Serialize, Deserialize)]
struct Res {
    data: Vec<Account>,
}

pub fn sign_up(_req: HttpRequest) -> HttpResponse {
    let connection = establish_connection();
    let account_entity = AccountEntity { connection };
    let new_account = account_entity.create("".to_string(), "".to_string(), "".to_string());

    match new_account {
        Ok(account) => {
            let res = serde_json::to_string(&account);

            match res {
                Ok(v) => return HttpResponse::Ok().content_type("application/json").body(v),
                Err(_e) => return ApiErrorResponse::AuthenticationFailure.error_response(),
            }
        }
        Err(_e) => return ApiErrorResponse::AuthenticationFailure.error_response(),
    }
}
