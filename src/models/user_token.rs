use chrono::Utc;
use jsonwebtoken::EncodingKey;

use super::user::LoginInfoDto;

pub static KEY: &[u8] = include_str!("../secret.key").as_bytes();
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub user: String,
    pub login_session: String,
}

impl UserToken {
    //generate token
    pub fn generate_token(login: &LoginInfoDto) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000;
        let pay_load = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.username.clone(),
            login_session: login.login_session.clone(),
        };

        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &pay_load,
            &EncodingKey::from_secret(KEY),
        )
        .unwrap()
    }
}
