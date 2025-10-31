use actix_web::{Error, HttpResponse};
use actix_web::http::header::LOCATION;

pub fn e500<T>(e: T) -> Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}
