use rocket::Rocket;

pub mod zones;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::ContentType;
    use rocket::local::Client;
    use rocket::Rocket;

    fn create_rocket_with_mounts() -> Rocket {
        let rocket = rocket::ignite();
        mount(rocket)
    }

    #[test]
    fn get_index_returns_content() {
        let client = Client::new(create_rocket_with_mounts()).unwrap();
        let mut response = client.get("/").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        assert_eq!("Hello, World!", body);
    }
}
