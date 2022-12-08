use actix_web::{http::StatusCode, HttpResponse};

use crate::models::response::ResponseBody;

#[derive(Debug)]
pub struct ServiceError {
    pub body: ResponseBody<String>,
    pub http_status: StatusCode,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(),
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}
