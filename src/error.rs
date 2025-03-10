use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    DatabaseError,
    InvalidPayload,
    InternalServerError,
    SessionError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "LOGIN_FAILED").into_response(),
            Error::DatabaseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR").into_response()
            }
            Error::InvalidPayload => (StatusCode::BAD_REQUEST, "INVALID_PAYLOAD").into_response(),
            Error::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR").into_response()
            }
            Error::SessionError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "SESSION_ERROR").into_response()
            }
        }
    }
}
