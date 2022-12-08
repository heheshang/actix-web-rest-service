use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    constants,
    schema::users::{self, dsl::*},
};

use super::{login_history::LoginHistory, user_token::UserToken};
#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UserDto {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub username_or_eamil: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct LoginInfoDto {
    pub username: String,
    pub login_session: String,
}

impl User {
    pub fn signup(user: UserDto, conn: &mut Connection) -> Result<String, String> {
        if Self::find_by_username(&user.username, conn).is_ok() {
            Err("Username already exists".to_string())
        } else {
            let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
            let user = UserDto {
                password: hashed_pwd,
                ..user
            };
            diesel::insert_into(users).values(&user).execute(conn);
            Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
        }
    }

    pub fn login(login: LoginDto, conn: &mut Connection) -> Option<LoginInfoDto> {
        let user_to_verify = users
            .filter(username.eq(&login.username_or_eamil))
            .or_filter(email.eq(&login.username_or_eamil))
            .get_result::<User>(conn);
        match user_to_verify {
            Ok(user) => {
                if !user.password.is_empty()
                    && bcrypt::verify(&login.password, &user.password).unwrap()
                {
                    if let Some(login_history) = LoginHistory::create(&user.username, conn) {
                        if LoginHistory::save_login_history(login_history, conn).is_ok() {
                            let login_session_str = User::generate_login_session();
                            if User::update_login_session(&user.username, &login_session_str, conn)
                            {
                                let login_info = LoginInfoDto {
                                    username: user.username,
                                    login_session: login_session_str,
                                };
                                return Some(login_info);
                            }
                        } else {
                            return None;
                        }
                    };

                    todo!()
                    // Some(login_info)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
    // logout function
    pub fn logout(user_id: i32, conn: &mut Connection) {
        if let Ok(user) = users.find(user_id).get_result::<User>(conn) {
            Self::update_login_session(&user.username, "", conn);
        }
    }
    // validate login session
    pub fn validate_login_session(user_token: &UserToken, conn: &mut Connection) -> bool {
        let user = users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn);
        user.is_ok()
    }

    // find user by username or email
    pub fn find_by_username(un: &str, conn: &mut Connection) -> QueryResult<User> {
        users.filter(username.eq(un)).get_result::<User>(conn)
    }

    // generate login session
    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_string()
    }

    // update login session
    pub fn update_login_session(un: &str, login_session_str: &str, conn: &mut Connection) -> bool {
        let user = Self::find_by_username(un, conn);
        match user {
            Ok(user) => diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok(),
            Err(_) => false,
        }
    }
}
