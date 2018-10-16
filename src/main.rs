#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod api;

fn main() {
    let rocket = rocket::ignite();
    let rocket = api::mount(rocket);
    let rocket = api::zones::mount(rocket);
    rocket.launch();
}
