use crate::error::Error;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde_json::Value;

#[derive(Serialize)]
pub struct Context {
    pub data: Value,
}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = Error;

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let context = Context {
            data: Value::default(),
        };

        Outcome::Success(context)
    }
}
