use actix_web::{web, HttpResponse, Result};

use crate::{
    config::db::Pool,
    constants,
    models::{filters::PersonFilter, preson::PersonDTO, response::ResponseBody},
    services::account_book_service,
};

pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_book_service::find_all(&pool) {
        Ok(persons) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, persons)))
        }
        Err(err) => Ok(err.response()),
    }
}

pub async fn find_by_id(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_book_service::find_by_id(id.into_inner(), &pool) {
        Ok(person) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, person))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn filter(
    web::Query(filter): web::Query<PersonFilter>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match account_book_service::filter(filter, &pool) {
        Ok(page) => Ok(HttpResponse::Ok().json(page)),
        Err(err) => Ok(err.response()),
    }
}

pub async fn insert(person: web::Json<PersonDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_book_service::insert(person.into_inner(), &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn update(
    id: web::Path<i32>,
    person: web::Json<PersonDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match account_book_service::update(id.into_inner(), person.into_inner(), &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}
pub async fn delete(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_book_service::delete(id.into_inner(), &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}
