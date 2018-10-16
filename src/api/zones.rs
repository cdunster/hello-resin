use rocket::Rocket;
use rocket_contrib::Json;

static ZONE_UUIDS: [&str; 2] = ["test-uuid-123", "different-uuid-456"];
static ZONE_NAMES: [&str; 2] = ["Zone Name", "Different Name"];

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/zones", routes![get_zones, get_zone_from_uuid])
}

#[get("/")]
fn get_zones() -> Json {
    Json(json!({
        "zones": []
    }))
}

#[get("/<uuid>")]
fn get_zone_from_uuid(uuid: String) -> Option<Json> {
    if let Some(i) = ZONE_UUIDS.iter().position(|&zone_uuid| zone_uuid == uuid) {
        Some(Json(json!({ "name": String::from(ZONE_NAMES[i]) })))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::{Client, LocalResponse};
    use rocket_contrib::Json;

    fn create_client_with_mounts() -> Client {
        let rocket = rocket::ignite();
        let rocket = mount(rocket);
        Client::new(rocket).unwrap()
    }

    #[test]
    fn given_no_zones_when_get_zones_then_return_json_object_with_empty_array() {
        let client = create_client_with_mounts();
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
        let client = create_client_with_mounts();
        let zone_uuid = "test-uuid-123";
        let zone_name = "Zone Name";
        let body = get_zone_return_response_body_string(&client, zone_uuid);

        let expected = Json(json!({ "name": zone_name })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn given_zones_when_get_zones_individually_then_return_correct_json_zone_object_each_time() {
        let client = create_client_with_mounts();

        let zone_uuid = "test-uuid-123";
        let zone_name = "Zone Name";
        let body = get_zone_return_response_body_string(&client, zone_uuid);

        let expected = Json(json!({ "name": zone_name })).to_string();
        assert_eq!(expected, body);

        let zone_uuid = "different-uuid-456";
        let zone_name = "Different Name";
        let body = get_zone_return_response_body_string(&client, zone_uuid);

        let expected = Json(json!({ "name": zone_name })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn given_none_existing_uuid_when_get_zone_then_return_error_not_found() {
        let client = create_client_with_mounts();

        let zone_uuid = "none-existing-uuid";
        let response = get_zone_return_response(&client, zone_uuid);

        assert_eq!(Status::NotFound, response.status());
    }
}
