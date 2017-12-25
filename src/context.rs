use rocket::request::{self, Request, FromRequest};
use database::{PgPool, PgPooledConnection};
use rocket::outcome::IntoOutcome;
use std::ops::Try;
use rocket::{State, Outcome};
use rocket::http::Status;
use error::Error;

pub struct Context {
    pub db: PgPooledConnection,
}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = Error;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let db = State::<PgPool>::from_request(req)
            .into_result()
            .map_err(|_| Error::Request("Unable to locate database pool"))
            .and_then(|pool| pool.get().map_err(|e| e.into()))
            .into_outcome(Status::ServiceUnavailable)?;

        let context = Context {
            db
        };

        Outcome::Success(context)
    }
}
