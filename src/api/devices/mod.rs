use device::{Device, DeviceCollection};
use rocket::{Rocket, State};
use rocket_contrib::{Json, UUID};
use std::sync::Mutex;

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
            ],
        ).manage(DeviceCollectionState::new(devices))
}

#[get("/", format = "application/json")]
fn get_devices(devices: State<DeviceCollectionState>) -> Json {
    Json(json!(devices.inner()))
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

#[patch("/<uuid>", format = "application/json", data = "<patch_json>")]
fn patch_device_from_uuid(uuid: UUID, patch_json: Json, devices: State<DeviceCollectionState>) -> Option<Json<Device>> {
    if let Some(device) = devices.lock().unwrap().get_mut(&uuid.into_inner()) {
        if let Some(patch_name) = patch_json["name"].as_str() {
            device.set_name(patch_name);
        }
        Some(Json(device.clone()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
