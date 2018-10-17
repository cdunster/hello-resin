use rocket::response::status;
use rocket::Rocket;
use rocket::State;
use rocket_contrib::Json;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;

struct Zone {
    name: &'static str,
}

impl Serialize for Zone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Zone", 1)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

pub struct ZoneCollection {
    zones: HashMap<&'static str, Zone>,
}

impl ZoneCollection {
    pub fn new() -> ZoneCollection {
        ZoneCollection {
            zones: HashMap::new(),
        }
    }

    fn add(&mut self, uuid: &'static str, name: &'static str) {
        self.zones.insert(uuid, Zone { name });
    }

    fn get(&self, uuid: &str) -> Option<&Zone> {
        self.zones.get(uuid)
    }
}

impl Serialize for ZoneCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ZoneCollection", 1)?;
        state.serialize_field("zones", &self.zones)?;
        state.end()
    }
}

pub fn mount(rocket: Rocket, zones: ZoneCollection) -> Rocket {
    rocket
        .mount("/zones", routes![get_zones, put_zones, get_zone_from_uuid])
        .manage(zones)
}

#[get("/")]
fn get_zones(zones: State<ZoneCollection>) -> Json {
    Json(json!(zones.inner()))
}

#[put("/")]
fn put_zones(zones: State<ZoneCollection>) -> status::Created<&str> {
    status::Created("/zones/new-uuid".to_string(), Some(""))
}

#[get("/<uuid>")]
fn get_zone_from_uuid(uuid: String, zones: State<ZoneCollection>) -> Option<Json> {
    if let Some(zone) = zones.get(&uuid) {
        Some(Json(json!(zone)))
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
    fn given_no_zones_when_get_zones_then_return_empty_json_object() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let mut response = client.get("/zones").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "zones": {}
        })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn given_zones_when_get_zones_then_return_json_object_with_uuids_mapped_to_zones() {
        let mut zones = ZoneCollection::new();
        let zone1_uuid = "test-uuid-123";
        let zone1_name = "Zone Name";
        let zone2_uuid = "different-uuid-456";
        let zone2_name = "Different Name";
        zones.add(zone1_uuid, zone1_name);
        zones.add(zone2_uuid, zone2_name);
        let client = create_client_with_mounts(zones);

        let mut response = client.get("/zones").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "zones": {
                zone1_uuid: {
                    "name": zone1_name
                },
                zone2_uuid: {
                    "name": zone2_name
                }
            }
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
        zones.add(zone_uuid, zone_name);
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
        zones.add(zone1_uuid, zone1_name);
        zones.add(zone2_uuid, zone2_name);
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

    fn put_zone_return_response<'c>(client: &'c Client, zone: &Zone) -> LocalResponse<'c> {
        client
            .put("/zones")
            .body(Json(json!(zone)).to_string())
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn when_put_zone_then_get_201_response() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let name = "Living Room";
        let zone = Zone { name };

        let response = put_zone_return_response(&client, &zone);

        assert_eq!(Status::Created, response.status());
    }
}
