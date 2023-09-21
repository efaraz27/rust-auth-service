use diesel::prelude::*;
use crate::models::{Client, NewClient};
use crate::schema::clients;

pub fn create_client(conn: &mut PgConnection, name: String, secret: String, redirect_uri: String) -> Result<Client, diesel::result::Error> {

    let new_client = NewClient {
        name,
        secret,
        redirect_uri,
    };

    diesel::insert_into(clients::table)
        .values(&new_client)
        .returning(clients::all_columns)
        .get_result(conn)
}

pub fn find_client_by_id(conn: &mut PgConnection, id: i32) -> Result<Client, diesel::result::Error> {
    clients::table
        .filter(clients::id.eq(id))
        .first(conn)
}