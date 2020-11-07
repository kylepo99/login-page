use actix_web::{
    dev::HttpResponseBuilder, error, get, http::header, http::StatusCode, App, HttpResponse,
};
use postgres::{Error};

use derive_more::{Display, Error};
use std::option::NoneError;
#[derive(Debug, Display, Error)]
pub enum MyError {
    #[display(fmt = "An Internal Error has occured")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,


    #[display(fmt = "A Internal Database error has occured")]
    DataBaseError,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::DataBaseError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

impl From<postgres::Error> for MyError {
    fn from(_: postgres::Error) -> Self {
        MyError::DataBaseError
    }
}

impl From<NoneError> for MyError {
    fn from(_: NoneError) -> Self {
        MyError::InternalError
    }
}
