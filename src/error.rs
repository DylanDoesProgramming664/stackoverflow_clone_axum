use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingQuestionId { id: Uuid },
    MissingAnswerId { id: Uuid },
    DBError(Box<dyn std::error::Error>),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("DEBUG: {:<12} - {self:?}", "INTO_RES");

        return (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response();
    }
}
