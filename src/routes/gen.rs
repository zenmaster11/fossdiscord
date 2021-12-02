extern crate backend;

use serde_derive::Serialize;
use rocket_contrib::json::Json;
use diesel;

use crate::lib::connect;
use crate::models::user;
use crate::models::user::User;
use crate::schema::users::dsl::*;

use diesel::prelude::*;

#[derive(Serialize)]
pub struct GenericOkMessage {
    code: i32,
    message: String
}

#[derive(Serialize)]
pub struct UserPayload {
    id: i32,
    username: String,
    discriminator: i32,
    unixCreationTime: i32
}

#[get("/")]
pub fn index() -> Json<GenericOkMessage> {
    Json(GenericOkMessage { code: 200, message: "Hello, world!".to_string() })
}

#[get("/users/<num>")]
pub fn userGet(num: i32) -> Json<UserPayload> {
    let connection = connect();
    let inst = users.filter(id.eq(num)).first::<User>(&connection).unwrap_or_else(|_|User {
        id: -1,
        username: "a3802".to_string(),
        password: "".to_string(), // somehow hide this just to get rid of confusion
        discriminator: 0000,
        unixCreationTime: 0
    }); // placeholder response
    
    Json(UserPayload {
        id: inst.id,
        username: inst.username,
        discriminator: inst.discriminator,
        unixCreationTime: inst.unixCreationTime
    })
}
