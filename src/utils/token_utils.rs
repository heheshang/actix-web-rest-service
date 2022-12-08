use actix_web::web::{self};
use jsonwebtoken::TokenData;

use crate::{
    config::db::Pool,
    models::{user::User, user_token::UserToken},
};

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(&[0; 16]),
        &jsonwebtoken::Validation::default(),
    )
}
pub fn verify_token(
    token_data: &TokenData<UserToken>,
    pool: &web::Data<Pool>,
) -> Result<String, String> {
    if User::validate_login_session(&token_data.claims, &mut pool.get().unwrap()) {
        Ok(token_data.claims.user.to_string())
    } else {
        Err("".to_string())
    }
}
