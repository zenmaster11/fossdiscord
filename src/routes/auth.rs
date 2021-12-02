
use crate::generic::{GenericResponseMessage, GenericErrorMessage};

#[get("/")]
pub fn index() -> Json<GenericResponseMessage> {
    Json(GenericResponseMessage {
        code: 200,
        message: "Auth API responsive." // this will probably never change
    })
}


#[post("/register")]
pub fn register() -> Json<GenericErrorMessage> {
    
}
