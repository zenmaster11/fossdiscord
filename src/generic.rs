use serde_derive::Serialize;

#[derive(Serialize)]
pub struct GenericResponseMessage {
    pub code: i32,
    pub message: String
}

#[derive(Serialize)]
pub struct GenericErrorMessage {
    pub error: bool,
    pub message: String
}
