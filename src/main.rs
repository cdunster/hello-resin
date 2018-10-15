#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::Json;

static ZONE_UUIDS: [&str; 2] = ["test-uuid-123", "different-uuid-456"];

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

#[get("/zones/<uuid>")]
fn get_zone_from_uuid(uuid: String) -> Option<Json> {
    if !ZONE_UUIDS.contains(&uuid.as_str()) {
        None
    } else {
        Some(Json(json!({})))
    }
}

fn create_rocket_with_mounts() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, get_zones, get_zone_from_uuid])
}

fn main() {
    create_rocket_with_mounts().launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::{Client, LocalResponse};

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

    fn get_zone_return_response_body_string(client: &Client, zone_uuid: &str) -> String {
        let mut response = client
            .get(format!("/zones/{}", zone_uuid))
            .header(ContentType::JSON)
            .dispatch();
        response.body_string().unwrap()
    }

    fn get_zone_return_response<'c>(client: &'c Client, zone_uuid: &str) -> LocalResponse<'c> {
        client
            .get(format!("/zones/{}", zone_uuid))
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn given_valid_uuid_when_get_single_zone_then_return_correct_json_zone_object() {
        let client = Client::new(create_rocket_with_mounts()).unwrap();
        let body = get_zone_return_response_body_string(&client, "test-uuid-123");

        let expected = Json(json!({})).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn given_zones_when_get_zones_individually_then_return_correct_json_zone_object_each_time() {
        let client = Client::new(create_rocket_with_mounts()).unwrap();

        let zone_uuid = "test-uuid-123";
        let body = get_zone_return_response_body_string(&client, zone_uuid);

        let expected = Json(json!({})).to_string();
        assert_eq!(expected, body);

        let zone_uuid = "different-uuid-456";
        let body = get_zone_return_response_body_string(&client, zone_uuid);

        let expected = Json(json!({})).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn given_none_existing_uuid_when_get_zone_then_return_error_not_found() {
        let client = Client::new(create_rocket_with_mounts()).unwrap();

        let zone_uuid = "none-existing-uuid";
        let response = get_zone_return_response(&client, zone_uuid);

        assert_eq!(Status::NotFound, response.status());
    }
}
