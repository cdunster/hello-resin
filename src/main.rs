#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn create_rocket_with_mounts() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    create_rocket_with_mounts().launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::ContentType;
    use rocket::local::Client;

    #[test]
    fn get_index_returns_content() {
        let client = Client::new(create_rocket_with_mounts()).unwrap();
        let mut response = client.get("/").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        assert_eq!("Hello, World!", body);
    }
}
