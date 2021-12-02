use diesel::{Queryable, Insertable};
use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub discriminator: i32,
    pub unixCreationTime: i32
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub discriminator: i32,
    pub unixCreationTime: i32
}
