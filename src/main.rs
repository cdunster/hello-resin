#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!\nThis page is brought to you from my RPi3."
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
