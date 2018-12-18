#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
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
mod zone;

fn main() {
    let zones = zone::ZoneCollection::new();
    let devices = device::DeviceCollection::new();

    let rocket = rocket::ignite();
    let rocket = api::mount(rocket);
    let rocket = api::zones::mount(rocket, zones);
    let rocket = api::devices::mount(rocket, devices);
    rocket.launch();
}
