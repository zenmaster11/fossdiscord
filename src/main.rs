#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
mod routes;
mod lib;
mod models;
mod schema;
mod generic;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::gen::index, routes::gen::userGet])
        .mount("/auth", routes![routes::auth::index, routes::auth::register])
        .launch();
}
