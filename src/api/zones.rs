use rocket::response::{self, status, Responder};
use rocket::{Request, Response, Rocket, State};
use rocket_contrib::Json;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Mutex;

#[derive(Clone, Copy)]
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

impl<'r> Responder<'r> for Zone {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{{\"name\":\"{}\"}}", self.name)))
            .ok()
    }
}

type ZoneCollectionState = Mutex<ZoneCollection>;

pub struct ZoneCollection {
    zones: HashMap<&'static str, Zone>,
}

impl ZoneCollection {
    pub fn new() -> ZoneCollection {
        ZoneCollection {
            zones: HashMap::new(),
        }
    }

    fn add(&mut self, uuid: &'static str, zone: Zone) {
        self.zones.insert(uuid, zone);
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
        .manage(ZoneCollectionState::new(zones))
}

#[get("/")]
fn get_zones(zones: State<ZoneCollectionState>) -> Json {
    Json(json!(zones.inner()))
}

#[put("/")]
fn put_zones(zones: State<ZoneCollectionState>) -> status::Created<Zone> {
    let zone = Zone {
        name: "Living Room",
    };
    zones.lock().unwrap().add("new-uuid", zone);

    status::Created("/zones/new-uuid".to_string(), Some(zone))
}

#[get("/<uuid>")]
fn get_zone_from_uuid(uuid: String, zones: State<ZoneCollectionState>) -> Option<Json> {
    if let Some(zone) = zones.lock().unwrap().get(&uuid) {
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
    use rocket_contrib::{Json, Value};
    use serde_json::map::Values;

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
        zones.add(zone1_uuid, Zone { name: zone1_name });
        zones.add(zone2_uuid, Zone { name: zone2_name });
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
        zones.add(zone_uuid, Zone { name: zone_name });
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
        zones.add(zone1_uuid, Zone { name: zone1_name });
        zones.add(zone2_uuid, Zone { name: zone2_name });
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

    #[test]
    fn when_put_zone_then_response_contains_new_zone_uri() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let name = "Living Room";
        let zone = Zone { name };

        let response = put_zone_return_response(&client, &zone);
        let response_uri = response.headers().get_one("Location").unwrap();

        assert!(response_uri.starts_with("/zones/"));
    }

    #[test]
    fn when_put_zone_then_response_body_contains_new_zone() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let name = "Living Room";
        let zone = Zone { name };

        let mut response = put_zone_return_response(&client, &zone);
        println!("{:?}", response);
        let body = response.body_string().unwrap();

        let expected = Json(json!({ "name": zone.name })).to_string();
        assert_eq!(expected, body);
    }

    fn get_zone_with_name<'z>(name: &str, zones: &'z mut Values) -> Option<&'z Value> {
        zones.find(|&zone| zone.get("name").unwrap() == name)
    }

    #[test]
    fn when_put_zone_then_new_zone_added() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let name = "Living Room";
        let zone = Zone { name };

        put_zone_return_response(&client, &zone);

        let mut response = client.get("/zones").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let body: Value = serde_json::from_str(&body).unwrap();
        let mut zones = body["zones"].as_object().unwrap().values();

        let zone = get_zone_with_name(name, &mut zones);

        assert!(zone.is_some());
    }
}
