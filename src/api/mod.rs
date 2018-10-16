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

    fn create_client_with_mounts() -> Client {
        let rocket = rocket::ignite();
        let rocket = mount(rocket);
        Client::new(rocket).unwrap()
    }

    #[test]
    fn get_index_returns_content() {
        let client = create_client_with_mounts();
        let mut response = client.get("/").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        assert_eq!("Hello, World!", body);
    }
}
