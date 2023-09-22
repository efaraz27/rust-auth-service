use diesel::prelude::*;
use diesel::result::Error;
use crate::models::{User, NewUser};
use crate::schema::users;

pub fn create_user(conn: &mut PgConnection, email: &String, password_hash: &String) -> Result<User, Error> {

    let new_user = NewUser {
        email: email.clone(),
        password_hash: password_hash.clone(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(users::all_columns)
        .get_result(conn)
}

pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> Result<User, Error> {
    users::table
        .filter(users::email.eq(email))
        .first(conn)
}
