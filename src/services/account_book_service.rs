use actix_web::{http::StatusCode, web};

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        filters::PersonFilter,
        preson::{Person, PersonDTO},
        response::Page,
    },
};

// find all function
pub fn find_all(poll: &web::Data<Pool>) -> Result<Vec<Person>, ServiceError> {
    match Person::find_all(&mut poll.get().unwrap()) {
        Ok(persons) => Ok(persons),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

// find by id function
pub fn find_by_id(id: i32, poll: &web::Data<Pool>) -> Result<Person, ServiceError> {
    match Person::find_by_id(id, &mut poll.get().unwrap()) {
        Ok(person) => Ok(person),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("person with id {} not found", id),
        )),
    }
}

// filter function
pub fn filter(filter: PersonFilter, poll: &web::Data<Pool>) -> Result<Page<Person>, ServiceError> {
    match Person::filter(filter, &mut poll.get().unwrap()) {
        Ok(persons) => Ok(persons),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

// insert function
pub fn insert(person: PersonDTO, poll: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Person::insert(person, &mut poll.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

// update function
pub fn update(
    id: i32,
    update_person: PersonDTO,
    poll: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match Person::find_by_id(id, &mut poll.get().unwrap()) {
        Ok(_) => match Person::update(id, update_person, &mut poll.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("person with id {} not found", id),
        )),
    }
}

// delete function
pub fn delete(id: i32, poll: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Person::find_by_id(id, &mut poll.get().unwrap()) {
        Ok(_) => match Person::delete(id, &mut poll.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("person with id {} not found", id),
        )),
    }
}
