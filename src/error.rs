use crate::model::Response;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct BusinessError {
    err: anyhow::Error,
}

impl fmt::Display for BusinessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.err.fmt(f)
    }
}

impl ResponseError for BusinessError {
    fn error_response(&self) -> HttpResponse {
        use log::error;
        error!("error: {:}", self.err.to_string());
        let resp = Response::err(500, &self.err.to_string());
        HttpResponse::BadRequest().json(resp)
    }
}

impl From<anyhow::Error> for BusinessError {
    fn from(err: anyhow::Error) -> Self {
        BusinessError { err }
    }
}
