use num_traits::Zero;
use bigdecimal::BigDecimal;
use uuid::Uuid;
use chrono::NaiveDate;
use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromParam};

pub struct FormDecimal(pub BigDecimal);
pub struct FormDate(pub NaiveDate);
pub struct FormUuid(pub Uuid);

impl<'v> FromFormValue<'v> for FormDecimal {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<FormDecimal, &'v RawStr> {
        match BigDecimal::parse_bytes(&form_value.as_bytes(), 10) {
            Some(val) => Ok(FormDecimal(val)),
            _ => Ok(FormDecimal(BigDecimal::zero())),
        }
    }
}

impl<'v> FromFormValue<'v> for FormDate {
    type Error = &'v str;

    fn from_form_value(form_value: &'v RawStr) -> Result<FormDate, &'v str> {
        match NaiveDate::parse_from_str(&form_value, "%Y-%m-%d") {
            Ok(date) => Ok(FormDate(date)),
            _ => Err("Unable to parse date")
        }
    }
}

impl<'v> FromFormValue<'v> for FormUuid {
    type Error = &'v str;

    fn from_form_value(form_value: &'v RawStr) -> Result<FormUuid, &'v str> {
        match Uuid::parse_str(&form_value) {
            Ok(uuid) => Ok(FormUuid(uuid)),
            _ => Err("Unable to parse uuid")
        }
    }
}

impl<'v> FromParam<'v> for FormUuid {
    type Error = &'v str;

    fn from_param(param: &'v RawStr) -> Result<FormUuid, &'v str> {
        match Uuid::parse_str(&param) {
            Ok(uuid) => Ok(FormUuid(uuid)),
            _ => Err("Unable to parse uuid")
        }
    }
}
