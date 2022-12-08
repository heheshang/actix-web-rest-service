#[allow(unused_imports)]
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
#[cfg(not(test))]
pub type Connection = PgConnection;
#[cfg(not(test))]
pub fn migrate_and_config_db(url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
    // let _conn = pool.get().expect("Failed to get connection from pool.");
}

#[cfg(test)]
pub type Connection = diesel::SqliteConnection;
#[cfg(test)]
pub fn migrate_and_config_db(url: &str) -> Pool {
    use diesel::{sql_query, RunQueryDsl};

    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let mut conn = pool.get().expect("Failed to get connection from pool.");
    sql_query(r#"DROP TABLE IF EXISTS log_history;"#)
        .execute(&mut conn)
        .unwrap();
    sql_query(r#"DROP TABLE IF EXISTS users;"#)
        .execute(&mut conn)
        .unwrap();
    sql_query(r#"DROP TABLE IF EXISTS people;"#)
        .execute(&mut conn)
        .unwrap();
    sql_query(
        r#"CREATE TABLE people (
            id INTEGER PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            gender BOOLEAN NOT NULL,
            age INTEGER NOT NULL,
            address TEXT NOT NULL,
            phone TEXT NOT NULL,
            email TEXT NOT NULL
    );
    "#,
    )
    .execute(&mut conn)
    .unwrap();

    sql_query(
        r#"
    CREATE TABLE users (
        id INTEGER PRIMARY KEY NOT NULL,
        username TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        login_session TEXT NOT NULL DEFAULT ''
    );
    "#,
    )
    .execute(&mut conn)
    .unwrap();
    sql_query(
        r#"
        CREATE TABLE login_history (
            id INTEGER PRIMARY KEY NOT NULL,
            user_id INTEGER NOT NULL REFERENCES users(id),
            login_timestamp INTEGER NOT NULL
        );

    "#,
    )
    .execute(&mut conn)
    .unwrap();
    pool
}
