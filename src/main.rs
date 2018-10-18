#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

mod api;

fn main() {
    let zones = api::zones::ZoneCollection::new();

    let rocket = rocket::ignite();
    let rocket = api::mount(rocket);
    let rocket = api::zones::mount(rocket, zones);
    rocket.launch();
}
