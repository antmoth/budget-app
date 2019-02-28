use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;
use error::Error;
use serde_json::Value;

#[derive(Serialize)]
pub struct Context {
    pub data: Value,
}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = Error;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let context = Context {
            data: Value::default(),
        };

        Outcome::Success(context)
    }
}
