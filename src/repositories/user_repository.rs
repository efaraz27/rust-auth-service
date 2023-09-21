use diesel::prelude::*;
use crate::models::{User, NewUser};
use crate::schema::users;

pub fn create_user(conn: &mut PgConnection, email: String, hashed_password: String) -> Result<User, diesel::result::Error> {

    let new_user = NewUser {
        email,
        password_hash: hashed_password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(users::all_columns)
        .get_result(conn)
}

pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> Result<User, diesel::result::Error> {
    users::table
        .filter(users::email.eq(email))
        .first(conn)
}
