use device::{Device, DeviceCollection};
use rocket::response::status;
use rocket::{Rocket, State};
use rocket_contrib::{Json, UUID};
use std::sync::Mutex;
use uuid::Uuid;

type DeviceCollectionState = Mutex<DeviceCollection>;

#[derive(FromForm)]
struct DeviceQuery {
    zone_uuid: UUID,
}

pub fn mount(rocket: Rocket, devices: DeviceCollection) -> Rocket {
    rocket
        .mount(
            "/devices",
            routes![
                get_devices,
                get_devices_with_query,
                get_device_from_uuid,
                patch_device_from_uuid,
                post_device
            ],
        ).manage(DeviceCollectionState::new(devices))
}

#[get("/", format = "application/json")]
fn get_devices(devices: State<DeviceCollectionState>) -> Json {
    Json(json!(devices.inner()))
}

#[post("/", data = "<device>", format = "application/json")]
fn post_device(device: Json<Device>, devices: State<DeviceCollectionState>) -> status::Created<Json<Device>> {
    let mut devices = devices.lock().unwrap();
    let uuid = Uuid::new_v4();
    devices.add(uuid, device.clone());

    status::Created(format!("/devices/{}", uuid), Some(device))
}

#[get("/?<device_query>", format = "application/json")]
fn get_devices_with_query(
    device_query: DeviceQuery,
    devices: State<DeviceCollectionState>,
) -> Option<Json<DeviceCollection>> {
    let devices = devices.lock().unwrap();
    let devices = devices.get_all_with_zone(device_query.zone_uuid.into_inner());
    if let Some(devices) = devices {
        Some(Json(devices))
    } else {
        None
    }
}

#[get("/<uuid>", format = "application/json")]
fn get_device_from_uuid(uuid: UUID, devices: State<DeviceCollectionState>) -> Option<Json<Device>> {
    if let Some(device) = devices.lock().unwrap().get(&uuid.into_inner()) {
        Some(Json(device.clone()))
    } else {
        None
    }
}

fn patch_device_with_json(device: &mut Device, patch_json: &Json) {
    let patch_json = patch_json.as_object().unwrap();

    if patch_json.contains_key("name") {
        if let Some(patch_name) = patch_json["name"].as_str() {
            device.set_name(patch_name);
        }
    }

    if patch_json.contains_key("zone_uuid") {
        if let Some(patch_zone_uuid) = patch_json["zone_uuid"].as_str() {
            let uuid = Uuid::parse_str(patch_zone_uuid).unwrap();
            device.set_zone_uuid(Some(uuid));
        } else {
            device.set_zone_uuid(None);
        }
    }

    if patch_json.contains_key("setpoint") {
        if let Some(patch_setpoint) = patch_json["setpoint"].as_f64() {
            device.set_setpoint(patch_setpoint);
        }
    }
}

#[patch("/<uuid>", format = "application/json", data = "<patch_json>")]
fn patch_device_from_uuid(uuid: UUID, patch_json: Json, devices: State<DeviceCollectionState>) -> Option<Json<Device>> {
    if let Some(device) = devices.lock().unwrap().get_mut(&uuid.into_inner()) {
        patch_device_with_json(device, &patch_json);
        Some(Json(device.clone()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
