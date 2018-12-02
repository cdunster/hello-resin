#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

mod api;
mod device;

fn main() {
    let zones = api::zones::ZoneCollection::new();
    let devices = device::DeviceCollection::new();

    let rocket = rocket::ignite();
    let rocket = api::mount(rocket);
    let rocket = api::zones::mount(rocket, zones);
    let rocket = api::devices::mount(rocket, devices);
    rocket.launch();
}
