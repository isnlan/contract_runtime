mod request;

use crate::error;
use actix_web::HttpResponse;
pub use request::*;
use serde::{Deserialize, Serialize};

const SUCCESS_CODE: i32 = 0;
const SUCCESS_MSG: &str = "ok";
pub const SUCCESS_RESPONSE: &str = "success";

#[derive(Deserialize, Serialize)]
pub struct Response<T>
where
    T: Serialize,
{
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Self {
        Response {
            code: SUCCESS_CODE,
            message: SUCCESS_MSG.to_owned(),
            data: Some(data),
        }
    }

    pub fn json(&self) -> Result<HttpResponse, error::HttpResponseError> {
        Ok(HttpResponse::Ok().json(self))
    }
}

impl Response<()> {
    pub fn err(error: i32, message: &str) -> Self {
        Response {
            code: error,
            message: message.to_owned(),
            data: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ResponseList<T>
where
    T: Serialize,
{
    pub total: i64,
    pub list: Vec<T>,
}
