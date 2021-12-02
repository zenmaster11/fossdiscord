
extern crate bcrypt;

use crate::generic::{GenericResponseMessage, GenericErrorMessage};
use crate::lib::connect;
use crate::models::user;
use crate::models::user::User;
use crate::models::user::NewUser;

use diesel;
use diesel::prelude::*;

use rocket::request::Form;
use rocket_contrib::json::Json;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::time::{SystemTime, UNIX_EPOCH};


#[get("/")]
pub fn index() -> Json<GenericResponseMessage> {
    Json(GenericResponseMessage {
        code: 200,
        message: "Auth API responsive.".to_string() // this will probably never change
    })
}

#[derive(FromForm)]
pub struct RegisterForm {
    pub username: String,
    pub password: String
}

#[post("/register", data="<registerForm>")]
pub fn register(registerForm: Form<RegisterForm>) -> Json<GenericErrorMessage> {
    if(registerForm.username.chars().count() < 3 || registerForm.username.chars().count() > 20) {
        return Json(GenericErrorMessage {
            error: true,
            message: "Your username must be 3-20 characters long.".to_string()
        });
    }

    if(!registerForm.username.chars().all(char::is_alphanumeric)) {
        return Json(GenericErrorMessage {
            error: true,
            message: "Your username must consist of only alphanumeric characters.".to_string()
        }); 
    }

    if(registerForm.password.chars().count() < 8) { // annoying limitation and probably wont do shit but oh well
        return Json(GenericErrorMessage {
            error: true,
            message: "Your password must be at least 8 characters long.".to_string()
        });
    }

    let dbConn = connect();
    let userUsername = &registerForm.username; // ass solution
    let newUser = NewUser {
        username: userUsername.to_string(),
        password: hash("hunter2", DEFAULT_COST).unwrap(),
        discriminator: 0000,
        unixCreationTime: 0
    };

    use crate::schema::users::dsl::{users};
    diesel::insert_into(users)
            .values(&newUser)
            .execute(&dbConn)
            .expect("Error saving user");

    Json(GenericErrorMessage {
        error: false,
        message: "Nothing went wrong.".to_string()
    })
}
