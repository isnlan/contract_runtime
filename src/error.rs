use crate::model::Response;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum BusinessError {
    #[error("10001#Validation error on field: {field}")]
    ValidationError { field: String },
    #[error("10002#Argument error")]
    ArgumentError,
    #[error("10000#An internal error occurred. Please try again later.")]
    InternalError {
        #[source]
        source: anyhow::Error,
    },
}

impl BusinessError {
    fn to_code(&self) -> i32 {
        let code = &self.to_string()[0..5];
        code.parse().unwrap_or(-1)
    }

    fn to_message(&self) -> String {
        self.to_string()[6..].to_owned()
    }
}

impl ResponseError for BusinessError {
    fn error_response(&self) -> HttpResponse {
        use log::error;
        if let BusinessError::InternalError { source } = self {
            error!("internal error: {:}", source.to_string());
        };
        let resp = Response::err(self.to_code(), &self.to_message());
        HttpResponse::BadRequest().json(resp)
    }
}

impl From<anyhow::Error> for BusinessError {
    fn from(err: anyhow::Error) -> Self {
        BusinessError::InternalError {source:err }
    }
}
