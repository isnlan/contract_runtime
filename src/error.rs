use crate::model::Response;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;
use std::string::FromUtf8Error;

#[derive(Error, Debug)]
pub struct HttpResponseError(pub anyhow::Error);

impl fmt::Display for HttpResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ResponseError for HttpResponseError {
    fn error_response(&self) -> HttpResponse {
        use log::error;
        error!("error: {:}", self.0.to_string());
        let resp = Response::err(500, &self.0.to_string());
        HttpResponse::Ok().json(resp)
    }
}

impl From<anyhow::Error> for HttpResponseError {
    fn from(err: anyhow::Error) -> Self {
        HttpResponseError(err)
    }
}

impl From<FromUtf8Error> for HttpResponseError {
    fn from(err: FromUtf8Error) -> Self {
        HttpResponseError(err.into())
    }
}

impl From<std::io::Error> for HttpResponseError {
    fn from(err: std::io::Error) -> Self {
        HttpResponseError(err.into())
    }
}
