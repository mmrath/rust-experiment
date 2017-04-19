#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;


use core::routes;

mod core;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .manage(core::service::Service::new())
        .mount("/", routes![index])
        .mount("/api", routes())
        .launch();
}