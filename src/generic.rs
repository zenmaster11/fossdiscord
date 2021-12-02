use serde::Serialize;

#[derive(Serialize)]
struct GenericResponseMessage {
    code: i32,
    message: String
}

#[derive(Serialize)]
struct GenericErrorMessage {
    error: bool,
    message: String
}
