#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/zones")]
fn get_zones() -> Json {
    Json(json!({
        "zones": []
    }))
}

fn create_rocket_with_mounts() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, get_zones])
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

    #[test]
    fn given_no_zones_when_get_zones_then_return_json_object_with_empty_array() {
        let client = Client::new(create_rocket_with_mounts()).unwrap();
        let mut response = client.get("/zones").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "zones": []
        })).to_string();
        assert_eq!(expected, body);
    }
}
