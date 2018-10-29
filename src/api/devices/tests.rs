use super::*;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};
use rocket_contrib::Json;

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
