#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!\nThis page is brought to you from my RPi3.\nTesting Docker caching, again!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
