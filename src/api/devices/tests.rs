use super::*;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};
use rocket_contrib::{Json, Value};
use serde_json::map::Values;

fn create_client_with_mounts(devices: DeviceCollection) -> Client {
    let rocket = rocket::ignite();
    let rocket = mount(rocket, devices);
    Client::new(rocket).unwrap()
}

fn get_device_return_response_body_string(client: &Client, device_uuid: &str) -> String {
    let mut response = client
        .get(format!("/devices/{}", device_uuid))
        .header(ContentType::JSON)
        .dispatch();
    response.body_string().unwrap()
}

#[cfg(test)]
mod get_devices {
    use super::*;

    #[test]
    fn with_no_devices_returns_empty_json_object_with_devices_key() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {}
        })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn with_devices_returns_json_object_with_uuids_mapped_to_devices() {
        let mut devices_map: HashMap<Uuid, Device> = HashMap::new();
        let device1_uuid = "84fa1356-d5de-11e8-9f8b-f2801f1b9fd1";
        let device1_name = "Device Name";
        let device2_uuid = "88f573e2-d5de-11e8-9f8b-f2801f1b9fd1";
        let device2_name = "Different Name";
        devices_map.insert(
            Uuid::parse_str(device1_uuid).unwrap(),
            Device {
                name: device1_name.to_string(),
            },
        );
        devices_map.insert(
            Uuid::parse_str(device2_uuid).unwrap(),
            Device {
                name: device2_name.to_string(),
            },
        );

        let devices = DeviceCollection { devices: devices_map };

        let client = create_client_with_mounts(devices);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {
                device1_uuid: {
                    "name": device1_name
                },
                device2_uuid: {
                    "name": device2_name
                }
            }
        })).to_string();
        assert_eq!(expected, body);
    }
}

#[cfg(test)]
mod get_device {
    use super::*;

    fn get_device_return_response<'c>(client: &'c Client, device_uuid: &str) -> LocalResponse<'c> {
        client
            .get(format!("/devices/{}", device_uuid))
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn with_valid_uuid_returns_correct_json_device_object() {
        let mut devices_map: HashMap<Uuid, Device> = HashMap::new();
        let device_uuid = "84fa1356-d5de-11e8-9f8b-f2801f1b9fd1";
        let device_name = "Device Name";
        devices_map.insert(
            Uuid::parse_str(device_uuid).unwrap(),
            Device {
                name: device_name.to_string(),
            },
        );

        let devices = DeviceCollection { devices: devices_map };

        let client = create_client_with_mounts(devices);

        let body = get_device_return_response_body_string(&client, device_uuid);

        let expected = Json(json!({ "name": device_name })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn multiple_devices_returns_correct_json_device_object_each_time() {
        let mut devices_map: HashMap<Uuid, Device> = HashMap::new();
        let device1_uuid = "84fa1356-d5de-11e8-9f8b-f2801f1b9fd1";
        let device1_name = "Device Name";
        let device2_uuid = "88f573e2-d5de-11e8-9f8b-f2801f1b9fd1";
        let device2_name = "Different Name";
        devices_map.insert(
            Uuid::parse_str(device1_uuid).unwrap(),
            Device {
                name: device1_name.to_string(),
            },
        );
        devices_map.insert(
            Uuid::parse_str(device2_uuid).unwrap(),
            Device {
                name: device2_name.to_string(),
            },
        );

        let devices = DeviceCollection { devices: devices_map };

        let client = create_client_with_mounts(devices);

        let body = get_device_return_response_body_string(&client, device1_uuid);

        let expected = Json(json!({ "name": device1_name })).to_string();
        assert_eq!(expected, body);

        let body = get_device_return_response_body_string(&client, device2_uuid);

        let expected = Json(json!({ "name": device2_name })).to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn none_existing_uuid_returns_error_not_found() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);

        let device_uuid = "88f573e2-d5de-11e8-9f8b-f2801f1b9fd1";
        let response = get_device_return_response(&client, device_uuid);

        assert_eq!(Status::NotFound, response.status());
    }

    #[test]
    fn after_call_device_remains() {
        let mut devices_map: HashMap<Uuid, Device> = HashMap::new();
        let device_uuid = "84fa1356-d5de-11e8-9f8b-f2801f1b9fd1";
        let device_name = "Device Name";
        devices_map.insert(
            Uuid::parse_str(device_uuid).unwrap(),
            Device {
                name: device_name.to_string(),
            },
        );

        let devices = DeviceCollection { devices: devices_map };

        let client = create_client_with_mounts(devices);

        let body = get_device_return_response_body_string(&client, device_uuid);

        let expected = Json(json!({ "name": device_name })).to_string();
        assert_eq!(expected, body);

        let body = get_device_return_response_body_string(&client, device_uuid);

        let expected = Json(json!({ "name": device_name })).to_string();
        assert_eq!(expected, body);
    }

}

#[cfg(test)]
mod post_device {
    use super::*;

    fn post_device_return_response<'c>(client: &'c Client, device: &Device) -> LocalResponse<'c> {
        client
            .post("/devices")
            .body(Json(json!(device)).to_string())
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn returns_201_response() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let name = "Living Room";
        let device = Device { name: name.to_string() };

        let response = post_device_return_response(&client, &device);

        assert_eq!(Status::Created, response.status());
    }

    #[test]
    fn response_contains_new_device_uri() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let name = "Living Room";
        let device = Device { name: name.to_string() };

        let response = post_device_return_response(&client, &device);
        let mut response_uri = response.headers().get_one("Location").unwrap().to_string();

        let new_uuid = response_uri.split_off("/devices/".len());
        let new_uuid = Uuid::parse_str(&new_uuid);

        assert_eq!("/devices/", response_uri);
        assert!(new_uuid.is_ok());
    }

    #[test]
    fn response_body_contains_new_device() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let name = "Living Room";
        let device = Device { name: name.to_string() };

        let mut response = post_device_return_response(&client, &device);
        println!("{:?}", response);
        let body = response.body_string().unwrap();

        let expected = Json(json!({ "name": device.name })).to_string();
        assert_eq!(expected, body);
    }

    fn get_device_with_name<'z>(name: &str, devices: &'z mut Values) -> Option<&'z Value> {
        devices
            .inspect(|&device| println!("Found device: {}", device))
            .find(|&device| device.get("name").unwrap() == name)
    }

    #[test]
    fn adds_device() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let name = "Living Room";
        let device = Device { name: name.to_string() };

        post_device_return_response(&client, &device);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let body: Value = serde_json::from_str(&body).unwrap();
        let mut devices = body["devices"].as_object().unwrap().values();

        let device = get_device_with_name(name, &mut devices);

        assert!(device.is_some());
    }

    #[test]
    fn can_add_more_than_one_device() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let device1 = Device {
            name: "Bathroom".to_string(),
        };
        let device2 = Device {
            name: "Study".to_string(),
        };

        post_device_return_response(&client, &device1);
        post_device_return_response(&client, &device2);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let body: Value = serde_json::from_str(&body).unwrap();

        let mut devices = body["devices"].as_object().unwrap().values();
        let device = get_device_with_name(&device1.name, &mut devices);
        assert!(device.is_some());

        let mut devices = body["devices"].as_object().unwrap().values();
        let device = get_device_with_name(&device2.name, &mut devices);
        assert!(device.is_some());
    }
}

#[cfg(test)]
mod patch_device {
    use super::*;

    fn patch_device_return_response<'c>(client: &'c Client, uuid: Uuid, device_json: Json) -> LocalResponse<'c> {
        client
            .patch(format!("/devices/{}", uuid))
            .body(device_json.to_string())
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn returns_updated_device() {
        let mut devices_map: HashMap<Uuid, Device> = HashMap::new();
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name";
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name";
        devices_map.insert(
            device1_uuid,
            Device {
                name: device1_name.to_string(),
            },
        );
        devices_map.insert(
            device2_uuid,
            Device {
                name: device2_name.to_string(),
            },
        );

        let devices = DeviceCollection { devices: devices_map };

        let client = create_client_with_mounts(devices);

        let patched_name = "New device name".to_string();
        let patch_json = Json(json!({ "name": patched_name }));
        let mut response = patch_device_return_response(&client, device1_uuid, patch_json);

        let returned_device: Device = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        let expected_device = Device { name: patched_name };

        assert_eq!(expected_device, returned_device);
    }
    #[test]
    fn updates_device_collection() {
        let mut devices_map: HashMap<Uuid, Device> = HashMap::new();
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name";
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name";
        devices_map.insert(
            device1_uuid,
            Device {
                name: device1_name.to_string(),
            },
        );
        devices_map.insert(
            device2_uuid,
            Device {
                name: device2_name.to_string(),
            },
        );

        let devices = DeviceCollection { devices: devices_map };

        let client = create_client_with_mounts(devices);

        let patched_name = "New device name".to_string();
        let patch_json = Json(json!({ "name": patched_name }));
        patch_device_return_response(&client, device1_uuid, patch_json);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {
                device1_uuid.to_string(): {
                    "name": patched_name
                },
                device2_uuid.to_string(): {
                    "name": device2_name
                }
            }
        })).to_string();
        assert_eq!(expected, body);
    }
}

#[cfg(test)]
mod delete_device {
    use super::*;

    fn delete_device_return_response<'c>(client: &'c Client, uuid: Uuid) -> LocalResponse<'c> {
        client
            .delete(format!("/devices/{}", uuid))
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn with_device_returns_204_no_content() {
        let mut devices_map: HashMap<Uuid, Device> = HashMap::new();
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name";
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name";
        devices_map.insert(
            device1_uuid,
            Device {
                name: device1_name.to_string(),
            },
        );
        devices_map.insert(
            device2_uuid,
            Device {
                name: device2_name.to_string(),
            },
        );
        let devices = DeviceCollection { devices: devices_map };
        let client = create_client_with_mounts(devices);

        let response = delete_device_return_response(&client, device1_uuid);

        assert_eq!(Status::NoContent, response.status());
    }

    #[test]
    fn device_deleted() {
        let mut devices_map: HashMap<Uuid, Device> = HashMap::new();
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name";
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name";
        devices_map.insert(
            device1_uuid,
            Device {
                name: device1_name.to_string(),
            },
        );
        devices_map.insert(
            device2_uuid,
            Device {
                name: device2_name.to_string(),
            },
        );
        let devices = DeviceCollection { devices: devices_map };
        let client = create_client_with_mounts(devices);

        delete_device_return_response(&client, device1_uuid);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {
                device2_uuid.to_string(): {
                    "name": device2_name
                }
            }
        })).to_string();
        assert_eq!(expected, body);
    }

}
