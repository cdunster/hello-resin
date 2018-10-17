use rocket::Rocket;
use rocket::State;
use rocket_contrib::Json;
use std::collections::HashMap;

pub type ZoneCollection = HashMap<&'static str, &'static str>;

pub fn mount(rocket: Rocket, zones: ZoneCollection) -> Rocket {
    rocket
        .mount("/zones", routes![get_zones, get_zone_from_uuid])
        .manage(zones)
}

#[get("/")]
fn get_zones() -> Json {
    Json(json!({
        "zones": []
    }))
}

#[get("/<uuid>")]
fn get_zone_from_uuid(uuid: String, zones: State<ZoneCollection>) -> Option<Json> {
    if let Some(zone_name) = zones.get(uuid.as_str()) {
        Some(Json(json!({ "name": zone_name })))
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

    fn create_client_with_mounts(zones: ZoneCollection) -> Client {
        let rocket = rocket::ignite();
        let rocket = mount(rocket, zones);
        Client::new(rocket).unwrap()
    }

    #[test]
    fn given_no_zones_when_get_zones_then_return_json_object_with_empty_array() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
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
        let mut zones = ZoneCollection::new();
        let zone_uuid = "test-uuid-123";
        let zone_name = "Zone Name";
        zones.insert(zone_uuid, zone_name);
        let client = create_client_with_mounts(zones);

        let body = get_zone_return_response_body_string(&client, zone_uuid);

        let expected = Json(json!({ "name": zone_name })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn given_zones_when_get_zones_individually_then_return_correct_json_zone_object_each_time() {
        let mut zones = ZoneCollection::new();
        let zone1_uuid = "test-uuid-123";
        let zone1_name = "Zone Name";
        let zone2_uuid = "different-uuid-456";
        let zone2_name = "Different Name";
        zones.insert(zone1_uuid, zone1_name);
        zones.insert(zone2_uuid, zone2_name);

        let client = create_client_with_mounts(zones);

        let body = get_zone_return_response_body_string(&client, zone1_uuid);

        let expected = Json(json!({ "name": zone1_name })).to_string();
        assert_eq!(expected, body);

        let body = get_zone_return_response_body_string(&client, zone2_uuid);

        let expected = Json(json!({ "name": zone2_name })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn given_none_existing_uuid_when_get_zone_then_return_error_not_found() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);

        let zone_uuid = "none-existing-uuid";
        let response = get_zone_return_response(&client, zone_uuid);

        assert_eq!(Status::NotFound, response.status());
    }
}
