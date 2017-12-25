use diesel;
use r2d2;

use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Msg(String),
    Request(&'static str),

    NotFound,
    NotUnique,
    
    DatabasePool { cause: r2d2::Error },
    Database     { cause: diesel::result::Error },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Msg(ref msg)           => write!(f, "Error: {}", msg)?,
            Error::Request(ref msg)       => write!(f, "Request Error: {}", msg)?,

            Error::NotFound               => write!(f, "Not Found")?,
            Error::NotUnique              => write!(f, "Unique Violation")?,

            Error::DatabasePool {..}      => write!(f, "Database Pool Error")?,
            Error::Database {..}          => write!(f, "Database Error")?,
        };

        match self.cause() {
            Some(cause) => write!(f, " - Cause: {}", cause),
            _ => Ok(()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "No description defined"
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::DatabasePool { ref cause } => Some(cause),
            Error::Database     { ref cause } => Some(cause),
            _ => None,
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(err: &'a str) -> Error {
        Error::Msg(err.into())
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Error {
        Error::DatabasePool { cause: err }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Error {
        use diesel::result::Error::*;
        use diesel::result::DatabaseErrorKind as EK;

        match err {
            NotFound => Error::NotFound,
            DatabaseError(EK::UniqueViolation, _) => Error::NotUnique,
            _ => Error::Database { cause: err },
        }
    }
}
