use actix_web::{
    http::{header::HeaderValue, StatusCode},
    web,
};

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        user::{LoginDto, User, UserDto},
        user_token::UserToken,
    },
    utils::token_utils,
};

#[derive(Deserialize, Serialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: UserDto, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match User::signup(user, &mut pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(e) => Err(ServiceError::new(StatusCode::BAD_REQUEST, e)),
    }
}
pub fn login(login: LoginDto, pool: &web::Data<Pool>) -> Result<TokenBodyResponse, ServiceError> {
    match User::login(login, &mut pool.get().unwrap()) {
        Some(login_info) => {
            match serde_json::from_value(
                json!({"token": UserToken::generate_token(&login_info),"token_type": "bearer"}),
            ) {
                Ok(token_res) => {
                    if login_info.login_session.is_empty() {
                        Err(ServiceError::new(
                            StatusCode::UNAUTHORIZED,
                            constants::MESSAGE_LOGIN_FAILED.to_string(),
                        ))
                    } else {
                        Ok(token_res)
                    }
                }
                Err(_) => Err(ServiceError::new(
                    StatusCode::UNAUTHORIZED,
                    constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
                )),
            }
        }
        None => Err(ServiceError::new(
            StatusCode::UNAUTHORIZED,
            constants::MESSAGE_USER_NOT_FOUND.to_string(),
        )),
    }
}

pub fn logout(authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    if let Ok(authen_str) = authen_header.to_str() {
        if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(username) = token_utils::verify_token(&token_data, pool) {
                    if let Ok(user) = User::find_by_username(&username, &mut pool.get().unwrap()) {
                        User::logout(user.id, &mut pool.get().unwrap());
                        return Ok(());
                    }
                }
            }
        }
    }
    Err(ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
    ))
}
#[cfg(test)]
mod tests {

    use log::info;

    use crate::config;

    use super::*;

    #[actix_rt::test]
    async fn test_service_signup_ok() {
        let pool = config::db::migrate_and_config_db(":memory:");
        info!("pool: {:?}", pool);
        let web_data = web::Data::new(pool);
        let user = UserDto {
            username: "test".to_string(),
            password: "test".to_string(),
            email: "12222@qq.com".to_string(),
        };
        let res = signup(user, &web_data);
        assert_eq!(res.unwrap(), constants::MESSAGE_SIGNUP_SUCCESS.to_string());
    }
}
