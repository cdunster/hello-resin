#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!\nThis page is brought to you from my RPi3."
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
