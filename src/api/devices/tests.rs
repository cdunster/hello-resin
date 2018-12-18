use super::*;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};
use rocket_contrib::json::{Json, JsonValue};
use serde_json::map::Values;
use serde_json::Value;
use uuid::Uuid;

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

mod get_devices {
    use super::*;

    fn get_devices_with_query_return_response<'c>(client: &'c Client, zone_uuid: &str) -> LocalResponse<'c> {
        client
            .get(format!("/devices?zone_uuid={}", zone_uuid))
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn with_no_devices_returns_empty_json_object_with_devices_key() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {}
        }))
        .to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn with_devices_returns_json_object_with_uuids_mapped_to_devices() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name".to_string();
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name".to_string();
        let mut devices = DeviceCollection::new();
        let device1 = Device::new(device1_name.clone(), None);
        let device2 = Device::new(device2_name.clone(), None);
        devices.add(device1_uuid, device1);
        devices.add(device2_uuid, device2);

        let client = create_client_with_mounts(devices);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {
                device1_uuid.to_string(): {
                    "name": device1_name,
                    "setpoint": 16.0,
                    "zone_uuid": null
                },
                device2_uuid.to_string(): {
                    "name": device2_name,
                    "setpoint": 16.0,
                    "zone_uuid": null
                }
            }
        }))
        .to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn with_devices_and_valid_query_returns_correct_devices() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name".to_string();
        let device1_zone = Uuid::parse_str("c00727d8-eee8-4a0e-850e-b81a74440e78").unwrap();
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name".to_string();
        let device2_zone = Uuid::parse_str("92024abf-6f13-4e6f-b519-0176a16e4ee0").unwrap();
        let mut devices = DeviceCollection::new();
        devices.add(device1_uuid, Device::new(device1_name, Some(device1_zone)));
        devices.add(device2_uuid, Device::new(device2_name.clone(), Some(device2_zone)));
        let client = create_client_with_mounts(devices);

        let mut response = get_devices_with_query_return_response(&client, &device2_zone.to_string());
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {
                device2_uuid.to_string(): {
                    "name": device2_name,
                    "setpoint": 16.0,
                    "zone_uuid": device2_zone
                }
            }
        }))
        .to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn with_devices_but_invalid_query_returns_404_error() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name".to_string();
        let device1_zone = Uuid::parse_str("c00727d8-eee8-4a0e-850e-b81a74440e78").unwrap();
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name".to_string();
        let device2_zone = Uuid::parse_str("92024abf-6f13-4e6f-b519-0176a16e4ee0").unwrap();
        let mut devices = DeviceCollection::new();
        devices.add(device1_uuid, Device::new(device1_name, Some(device1_zone)));
        devices.add(device2_uuid, Device::new(device2_name, Some(device2_zone)));
        let client = create_client_with_mounts(devices);

        let zone_uuid = "690ad0c5-a04f-479f-9d1f-d076df3a2c7b";
        let response = get_devices_with_query_return_response(&client, zone_uuid);

        assert_eq!(Status::NotFound, response.status());
    }

    #[test]
    fn query_with_no_devices_returns_404_error() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);

        let zone_uuid = "690ad0c5-a04f-479f-9d1f-d076df3a2c7b";
        let response = get_devices_with_query_return_response(&client, zone_uuid);

        assert_eq!(Status::NotFound, response.status());
    }

    #[test]
    fn with_devices_and_valid_query_does_not_remove_devices() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name".to_string();
        let device1_zone = Uuid::parse_str("c00727d8-eee8-4a0e-850e-b81a74440e78").unwrap();
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name".to_string();
        let device2_zone = Uuid::parse_str("92024abf-6f13-4e6f-b519-0176a16e4ee0").unwrap();
        let mut devices = DeviceCollection::new();
        devices.add(device1_uuid, Device::new(device1_name.clone(), Some(device1_zone)));
        devices.add(device2_uuid, Device::new(device2_name.clone(), Some(device2_zone)));
        let client = create_client_with_mounts(devices);

        get_devices_with_query_return_response(&client, &device2_zone.to_string());
        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {
                device1_uuid.to_string(): {
                    "name": device1_name,
                    "setpoint": 16.0,
                    "zone_uuid": device1_zone
                },
                device2_uuid.to_string(): {
                    "name": device2_name,
                    "setpoint": 16.0,
                    "zone_uuid": device2_zone
                }
            }
        }))
        .to_string();
        assert_eq!(expected, body);
    }
}

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
        let device_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device_name = "Device Name".to_string();
        let mut devices = DeviceCollection::new();
        devices.add(device_uuid, Device::new(device_name.clone(), None));

        let client = create_client_with_mounts(devices);

        let body = get_device_return_response_body_string(&client, &device_uuid.to_string());

        let expected = Json(json!({
            "name": device_name,
            "setpoint": 16.0,
            "zone_uuid": null
        }))
        .to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn multiple_devices_returns_correct_json_device_object_each_time() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name".to_string();
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name".to_string();
        let mut devices = DeviceCollection::new();
        devices.add(device1_uuid, Device::new(device1_name.clone(), None));
        devices.add(device2_uuid, Device::new(device2_name.clone(), None));

        let client = create_client_with_mounts(devices);

        let body = get_device_return_response_body_string(&client, &device1_uuid.to_string());

        let expected = Json(json!({
        "name": device1_name,
        "setpoint": 16.0,
        "zone_uuid": null
        }))
        .to_string();
        assert_eq!(expected, body);

        let body = get_device_return_response_body_string(&client, &device2_uuid.to_string());

        let expected = Json(json!({
        "name": device2_name,
        "setpoint": 16.0,
        "zone_uuid": null
        }))
        .to_string();
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
        let device_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device_name = "Device Name".to_string();
        let mut devices = DeviceCollection::new();
        devices.add(device_uuid, Device::new(device_name.clone(), None));

        let client = create_client_with_mounts(devices);

        let body = get_device_return_response_body_string(&client, &device_uuid.to_string());

        let expected = Json(json!({
            "name": device_name,
            "setpoint": 16.0,
            "zone_uuid": null
        }))
        .to_string();
        assert_eq!(expected, body);

        let body = get_device_return_response_body_string(&client, &device_uuid.to_string());

        let expected = Json(json!({
            "name": device_name,
            "setpoint": 16.0,
            "zone_uuid": null
        }))
        .to_string();
        assert_eq!(expected, body);
    }

}

mod patch_device {
    use super::*;

    fn patch_device_return_response<'c>(client: &'c Client, uuid: Uuid, device_json: JsonValue) -> LocalResponse<'c> {
        client
            .patch(format!("/devices/{}", uuid))
            .body(device_json.to_string())
            .header(ContentType::JSON)
            .dispatch()
    }

    #[test]
    fn returns_updated_device() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name";
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name";
        let mut devices = DeviceCollection::new();
        devices.add(device1_uuid, Device::new(device1_name.to_string(), None));
        devices.add(device2_uuid, Device::new(device2_name.to_string(), None));

        let client = create_client_with_mounts(devices);

        let patched_name = "New device name".to_string();
        let patched_zone_uuid = Uuid::parse_str("b098d5ca-1311-4145-80b2-0e9b2944efd3").unwrap();
        let patch_json = json!({ "name": patched_name, "zone_uuid": patched_zone_uuid });
        let mut response = patch_device_return_response(&client, device1_uuid, patch_json);

        let returned_device: Device = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        let expected_device = Device::new(patched_name, Some(patched_zone_uuid));

        assert_eq!(expected_device, returned_device);
    }

    #[test]
    fn updates_device_collection() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name";
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name";

        let mut devices = DeviceCollection::new();
        devices.add(device1_uuid, Device::new(device1_name.to_string(), None));
        devices.add(device2_uuid, Device::new(device2_name.to_string(), None));

        let client = create_client_with_mounts(devices);

        let patched_name = "New device name".to_string();
        let patch_json = json!({ "name": patched_name });
        patch_device_return_response(&client, device1_uuid, patch_json);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {
                device1_uuid.to_string(): {
                    "name": patched_name,
                    "setpoint": 16.0,
                    "zone_uuid": null
                },
                device2_uuid.to_string(): {
                    "name": device2_name,
                    "setpoint": 16.0,
                    "zone_uuid": null
                }
            }
        }))
        .to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn can_remove_zone_uuid() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name";
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name";
        let zone_uuid = Uuid::parse_str("b098d5ca-1311-4145-80b2-0e9b2944efd3").unwrap();

        let mut devices = DeviceCollection::new();
        devices.add(device1_uuid, Device::new(device1_name.to_string(), Some(zone_uuid)));
        devices.add(device2_uuid, Device::new(device2_name.to_string(), Some(zone_uuid)));

        let client = create_client_with_mounts(devices);

        let patch_json = json!({ "zone_uuid": null });
        patch_device_return_response(&client, device1_uuid, patch_json);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "devices": {
                device1_uuid.to_string(): {
                    "name": device1_name,
                    "setpoint": 16.0,
                    "zone_uuid": null
                },
                device2_uuid.to_string(): {
                    "name": device2_name,
                    "setpoint": 16.0,
                    "zone_uuid": "b098d5ca-1311-4145-80b2-0e9b2944efd3"
                }
            }
        }))
        .to_string();
        assert_eq!(expected, body);
    }

    #[test]
    fn can_partial_patch_name_only() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name";
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name";
        let zone_uuid = Uuid::parse_str("b098d5ca-1311-4145-80b2-0e9b2944efd3").unwrap();
        let mut devices = DeviceCollection::new();
        devices.add(device1_uuid, Device::new(device1_name.to_string(), Some(zone_uuid)));
        devices.add(device2_uuid, Device::new(device2_name.to_string(), None));

        let client = create_client_with_mounts(devices);

        let patched_name = "New device name".to_string();
        let patch_json = json!({ "name": patched_name });
        let mut response = patch_device_return_response(&client, device1_uuid, patch_json);

        let returned_device: Device = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        let expected_device = Device::new(patched_name, Some(zone_uuid));

        assert_eq!(expected_device, returned_device);
    }

    #[test]
    fn can_set_setpoint() {
        let device1_uuid = Uuid::parse_str("84fa1356-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device1_name = "Device Name";
        let device2_uuid = Uuid::parse_str("88f573e2-d5de-11e8-9f8b-f2801f1b9fd1").unwrap();
        let device2_name = "Different Name";
        let zone_uuid = Uuid::parse_str("b098d5ca-1311-4145-80b2-0e9b2944efd3").unwrap();
        let mut devices = DeviceCollection::new();
        devices.add(device1_uuid, Device::new(device1_name.to_string(), Some(zone_uuid)));
        devices.add(device2_uuid, Device::new(device2_name.to_string(), None));

        let client = create_client_with_mounts(devices);

        let patch_setpoint = 22.3;
        let patch_json = json!({ "setpoint": patch_setpoint });
        let body = patch_device_return_response(&client, device1_uuid, patch_json)
            .body_string()
            .unwrap();

        let expected = Json(json!({
            "name": device1_name,
            "setpoint": patch_setpoint,
            "zone_uuid": zone_uuid
        }))
        .to_string();
        assert_eq!(expected, body);
    }
}

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
        let name = "Living Room".to_string();
        let device = Device::new(name, None);

        let response = post_device_return_response(&client, &device);

        assert_eq!(Status::Created, response.status());
    }

    #[test]
    fn response_contains_new_device_uri() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let name = "Living Room".to_string();
        let device = Device::new(name, None);

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
        let name = "Living Room".to_string();
        let device = Device::new(name.clone(), None);

        let mut response = post_device_return_response(&client, &device);
        println!("{:?}", response);
        let body = response.body_string().unwrap();

        let expected = Json(json!({
            "name": name,
            "setpoint": 16.0,
            "zone_uuid": null
        }))
        .to_string();
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
        let name = "Living Room".to_string();
        let device = Device::new(name.clone(), None);

        post_device_return_response(&client, &device);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let body: Value = serde_json::from_str(&body).unwrap();
        let mut devices = body["devices"].as_object().unwrap().values();

        let device = get_device_with_name(&name, &mut devices);

        assert!(device.is_some());
    }

    #[test]
    fn can_add_more_than_one_device() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let device1_name = "Bathroom".to_string();
        let device2_name = "Study".to_string();
        let device1 = Device::new(device1_name.clone(), None);
        let device2 = Device::new(device2_name.clone(), None);

        post_device_return_response(&client, &device1);
        post_device_return_response(&client, &device2);

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let body: Value = serde_json::from_str(&body).unwrap();

        let mut devices = body["devices"].as_object().unwrap().values();
        let device = get_device_with_name(&device1_name, &mut devices);
        assert!(device.is_some());

        let mut devices = body["devices"].as_object().unwrap().values();
        let device = get_device_with_name(&device2_name, &mut devices);
        assert!(device.is_some());
    }

    #[test]
    fn device_not_added_when_name_missing() {
        let devices = DeviceCollection::new();
        let client = create_client_with_mounts(devices);
        let zone_uuid = Uuid::parse_str("92024abf-6f13-4e6f-b519-0176a16e4ee0").unwrap();

        client
            .post("/devices")
            .body(Json(json!({ "zone_uuid": zone_uuid })).to_string())
            .header(ContentType::JSON)
            .dispatch();

        let mut response = client.get("/devices").header(ContentType::JSON).dispatch();
        let body = response.body_string().unwrap();

        let body: Value = serde_json::from_str(&body).unwrap();
        let devices = body["devices"].as_object().unwrap();

        assert!(devices.is_empty());
    }
}
