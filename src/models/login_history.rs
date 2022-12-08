use crate::{
    config::db::Connection,
    models::user::User,
    schema::login_history::{self, dsl::*},
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Identifiable, Associations, Queryable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = login_history)]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = login_history)]
pub struct LoginHistoryDto {
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

impl LoginHistory {
    pub fn create(un: &str, conn: &mut Connection) -> Option<LoginHistoryDto> {
        let user = User::find_by_username(un, conn);
        match user {
            Ok(user) => {
                let now = Utc::now().naive_utc();
                Some(LoginHistoryDto {
                    user_id: user.id,
                    login_timestamp: now,
                })
            }
            Err(_) => None,
        }
    }

    pub fn save_login_history(
        insert_record: LoginHistoryDto,
        conn: &mut Connection,
    ) -> QueryResult<usize> {
        diesel::insert_into(login_history)
            .values(&insert_record)
            .execute(conn)
    }
}
