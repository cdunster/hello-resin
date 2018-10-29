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

fn get_zone_return_response_body_string(client: &Client, zone_uuid: &str) -> String {
    let mut response = client
        .get(format!("/zones/{}", zone_uuid))
        .header(ContentType::JSON)
        .dispatch();
    response.body_string().unwrap()
}

mod get_zones {
    use super::*;

    #[test]
    fn with_no_zones_returns_empty_json_object_with_zones_key() {
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
    fn with_zones_returns_json_object_with_uuids_mapped_to_zones() {
        let mut zones_map: HashMap<Uuid, Zone> = HashMap::new();
        let zone1_uuid = "84fa1356-d5de-11e8-9f8b-f2801f1b9fd1";
        let zone1_name = "Zone Name";
        let zone2_uuid = "88f573e2-d5de-11e8-9f8b-f2801f1b9fd1";
        let zone2_name = "Different Name";
        zones_map.insert(
            Uuid::parse_str(zone1_uuid).unwrap(),
            Zone {
                name: zone1_name.to_string(),
            },
        );
        zones_map.insert(
            Uuid::parse_str(zone2_uuid).unwrap(),
            Zone {
                name: zone2_name.to_string(),
            },
        );

        let zones = ZoneCollection { zones: zones_map };

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
}

mod get_zone {
    use super::*;

    fn get_zone_return_response<'c>(client: &'c Client, zone_uuid: &str) -> LocalResponse<'c> {
        client
            .get(format!("/zones/{}", zone_uuid))
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn with_valid_uuid_returns_correct_json_zone_object() {
        let mut zones_map: HashMap<Uuid, Zone> = HashMap::new();
        let zone_uuid = "84fa1356-d5de-11e8-9f8b-f2801f1b9fd1";
        let zone_name = "Zone Name";
        zones_map.insert(
            Uuid::parse_str(zone_uuid).unwrap(),
            Zone {
                name: zone_name.to_string(),
            },
        );

        let zones = ZoneCollection { zones: zones_map };

        let client = create_client_with_mounts(zones);

        let body = get_zone_return_response_body_string(&client, zone_uuid);

        let expected = Json(json!({ "name": zone_name })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn multiple_zones_returns_correct_json_zone_object_each_time() {
        let mut zones_map: HashMap<Uuid, Zone> = HashMap::new();
        let zone1_uuid = "84fa1356-d5de-11e8-9f8b-f2801f1b9fd1";
        let zone1_name = "Zone Name";
        let zone2_uuid = "88f573e2-d5de-11e8-9f8b-f2801f1b9fd1";
        let zone2_name = "Different Name";
        zones_map.insert(
            Uuid::parse_str(zone1_uuid).unwrap(),
            Zone {
                name: zone1_name.to_string(),
            },
        );
        zones_map.insert(
            Uuid::parse_str(zone2_uuid).unwrap(),
            Zone {
                name: zone2_name.to_string(),
            },
        );

        let zones = ZoneCollection { zones: zones_map };

        let client = create_client_with_mounts(zones);

        let body = get_zone_return_response_body_string(&client, zone1_uuid);

        let expected = Json(json!({ "name": zone1_name })).to_string();
        assert_eq!(expected, body);

        let body = get_zone_return_response_body_string(&client, zone2_uuid);

        let expected = Json(json!({ "name": zone2_name })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn none_existing_uuid_returns_error_not_found() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);

        let zone_uuid = "88f573e2-d5de-11e8-9f8b-f2801f1b9fd1";
        let response = get_zone_return_response(&client, zone_uuid);

        assert_eq!(Status::NotFound, response.status());
    }

    #[test]
    fn after_call_zone_remains() {
        let mut zones_map: HashMap<Uuid, Zone> = HashMap::new();
        let zone_uuid = "84fa1356-d5de-11e8-9f8b-f2801f1b9fd1";
        let zone_name = "Zone Name";
        zones_map.insert(
            Uuid::parse_str(zone_uuid).unwrap(),
            Zone {
                name: zone_name.to_string(),
            },
        );

        let zones = ZoneCollection { zones: zones_map };

        let client = create_client_with_mounts(zones);

        let body = get_zone_return_response_body_string(&client, zone_uuid);

        let expected = Json(json!({ "name": zone_name })).to_string();
        assert_eq!(expected, body);

        let body = get_zone_return_response_body_string(&client, zone_uuid);

        let expected = Json(json!({ "name": zone_name })).to_string();
        assert_eq!(expected, body);
    }

}

mod post_zone {
    use super::*;

    fn post_zone_return_response<'c>(client: &'c Client, zone: &Zone) -> LocalResponse<'c> {
        client
            .post("/zones")
            .body(Json(json!(zone)).to_string())
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn returns_201_response() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let name = "Living Room";
        let zone = Zone { name: name.to_string() };

        let response = post_zone_return_response(&client, &zone);

        assert_eq!(Status::Created, response.status());
    }

    #[test]
    fn response_contains_new_zone_uri() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let name = "Living Room";
        let zone = Zone { name: name.to_string() };

        let response = post_zone_return_response(&client, &zone);
        let mut response_uri = response.headers().get_one("Location").unwrap().to_string();

        let new_uuid = response_uri.split_off("/zones/".len());
        let new_uuid = Uuid::parse_str(&new_uuid);

        assert_eq!("/zones/", response_uri);
        assert!(new_uuid.is_ok());
    }

    #[test]
    fn response_body_contains_new_zone() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let name = "Living Room";
        let zone = Zone { name: name.to_string() };

        let mut response = post_zone_return_response(&client, &zone);
        println!("{:?}", response);
        let body = response.body_string().unwrap();

        let expected = Json(json!({ "name": zone.name })).to_string();
        assert_eq!(expected, body);
    }

    fn get_zone_with_name<'z>(name: &str, zones: &'z mut Values) -> Option<&'z Value> {
        zones
            .inspect(|&zone| println!("Found zone: {}", zone))
            .find(|&zone| zone.get("name").unwrap() == name)
    }

    #[test]
    fn adds_zone() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let name = "Living Room";
        let zone = Zone { name: name.to_string() };

        post_zone_return_response(&client, &zone);

        let mut response = client.get("/zones").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let body: Value = serde_json::from_str(&body).unwrap();
        let mut zones = body["zones"].as_object().unwrap().values();

        let zone = get_zone_with_name(name, &mut zones);

        assert!(zone.is_some());
    }

    #[test]
    fn can_add_more_than_one_zone() {
        let zones = ZoneCollection::new();
        let client = create_client_with_mounts(zones);
        let zone1 = Zone {
            name: "Bathroom".to_string(),
        };
        let zone2 = Zone {
            name: "Study".to_string(),
        };

        post_zone_return_response(&client, &zone1);
        post_zone_return_response(&client, &zone2);

        let mut response = client.get("/zones").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let body: Value = serde_json::from_str(&body).unwrap();

        let mut zones = body["zones"].as_object().unwrap().values();
        let zone = get_zone_with_name(&zone1.name, &mut zones);
        assert!(zone.is_some());

        let mut zones = body["zones"].as_object().unwrap().values();
        let zone = get_zone_with_name(&zone2.name, &mut zones);
        assert!(zone.is_some());
    }
}

mod patch_zone {
    use super::*;

    fn patch_zone_return_response<'c>(client: &'c Client, uuid: Uuid, zone_json: Json) -> LocalResponse<'c> {
        client
            .patch(format!("/zones/{}", uuid))
            .body(zone_json.to_string())
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn returns_updated_zone() {
        let mut zones_map: HashMap<Uuid, Zone> = HashMap::new();
        let zone1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let zone1_name = "Zone Name";
        let zone2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let zone2_name = "Different Name";
        zones_map.insert(
            zone1_uuid,
            Zone {
                name: zone1_name.to_string(),
            },
        );
        zones_map.insert(
            zone2_uuid,
            Zone {
                name: zone2_name.to_string(),
            },
        );

        let zones = ZoneCollection { zones: zones_map };

        let client = create_client_with_mounts(zones);

        let patched_name = "New zone name".to_string();
        let patch_json = Json(json!({ "name": patched_name }));
        let mut response = patch_zone_return_response(&client, zone1_uuid, patch_json);

        let returned_zone: Zone = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        let expected_zone = Zone { name: patched_name };

        assert_eq!(expected_zone, returned_zone);
    }
    #[test]
    fn updates_zone_collection() {
        let mut zones_map: HashMap<Uuid, Zone> = HashMap::new();
        let zone1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let zone1_name = "Zone Name";
        let zone2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let zone2_name = "Different Name";
        zones_map.insert(
            zone1_uuid,
            Zone {
                name: zone1_name.to_string(),
            },
        );
        zones_map.insert(
            zone2_uuid,
            Zone {
                name: zone2_name.to_string(),
            },
        );

        let zones = ZoneCollection { zones: zones_map };

        let client = create_client_with_mounts(zones);

        let patched_name = "New zone name".to_string();
        let patch_json = Json(json!({ "name": patched_name }));
        patch_zone_return_response(&client, zone1_uuid, patch_json);

        let mut response = client.get("/zones").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "zones": {
                zone1_uuid.to_string(): {
                    "name": patched_name
                },
                zone2_uuid.to_string(): {
                    "name": zone2_name
                }
            }
        })).to_string();
        assert_eq!(expected, body);
    }
}

mod delete_zone {
    use super::*;

    fn delete_zone_return_response<'c>(client: &'c Client, uuid: Uuid) -> LocalResponse<'c> {
        client
            .delete(format!("/zones/{}", uuid))
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn with_zone_returns_204_no_content() {
        let mut zones_map: HashMap<Uuid, Zone> = HashMap::new();
        let zone1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let zone1_name = "Zone Name";
        let zone2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let zone2_name = "Different Name";
        zones_map.insert(
            zone1_uuid,
            Zone {
                name: zone1_name.to_string(),
            },
        );
        zones_map.insert(
            zone2_uuid,
            Zone {
                name: zone2_name.to_string(),
            },
        );
        let zones = ZoneCollection { zones: zones_map };
        let client = create_client_with_mounts(zones);

        let response = delete_zone_return_response(&client, zone1_uuid);

        assert_eq!(Status::NoContent, response.status());
    }

    #[test]
    fn zone_deleted() {
        let mut zones_map: HashMap<Uuid, Zone> = HashMap::new();
        let zone1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let zone1_name = "Zone Name";
        let zone2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let zone2_name = "Different Name";
        zones_map.insert(
            zone1_uuid,
            Zone {
                name: zone1_name.to_string(),
            },
        );
        zones_map.insert(
            zone2_uuid,
            Zone {
                name: zone2_name.to_string(),
            },
        );
        let zones = ZoneCollection { zones: zones_map };
        let client = create_client_with_mounts(zones);

        delete_zone_return_response(&client, zone1_uuid);

        let mut response = client.get("/zones").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "zones": {
                zone2_uuid.to_string(): {
                    "name": zone2_name
                }
            }
        })).to_string();
        assert_eq!(expected, body);
    }

}
