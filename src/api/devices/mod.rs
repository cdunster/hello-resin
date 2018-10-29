use rocket::response::status;
use rocket::{Rocket, State};
use rocket_contrib::{Json, UUID};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct Device {
    name: String,
}

type DeviceCollectionState = Mutex<DeviceCollection>;

#[derive(Serialize)]
pub struct DeviceCollection {
    devices: HashMap<Uuid, Device>,
}

impl DeviceCollection {
    pub fn new() -> DeviceCollection {
        DeviceCollection {
            devices: HashMap::new(),
        }
    }

    fn add(&mut self, device: Device) -> Entry<Uuid, Device> {
        let uuid = Uuid::new_v4();
        self.devices.insert(uuid, device);

        self.devices.entry(uuid)
    }

    fn get(&self, uuid: &Uuid) -> Option<&Device> {
        self.devices.get(uuid)
    }

    fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Device> {
        self.devices.get_mut(uuid)
    }

    fn remove(&mut self, uuid: &Uuid) {
        self.devices.remove(uuid);
    }
}

pub fn mount(rocket: Rocket, devices: DeviceCollection) -> Rocket {
    rocket
        .mount(
            "/devices",
            routes![
                get_devices,
                post_devices,
                get_device_from_uuid,
                patch_device_from_uuid,
                delete_device_from_uuid
            ],
        ).manage(DeviceCollectionState::new(devices))
}

#[get("/", format = "application/json")]
fn get_devices(devices: State<DeviceCollectionState>) -> Json {
    Json(json!(devices.inner()))
}

#[post("/", format = "application/json", data = "<device>")]
fn post_devices(device: Json<Device>, devices: State<DeviceCollectionState>) -> status::Created<Json<Device>> {
    let mut devices = devices.lock().unwrap();
    let device_entry = devices.add(device.clone());

    status::Created(format!("/devices/{}", device_entry.key()), Some(device))
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
            device.name = patch_name.to_string().clone();
        }
        Some(Json(device.clone()))
    } else {
        None
    }
}

#[delete("/<uuid>", format = "application/json")]
fn delete_device_from_uuid(uuid: UUID, devices: State<DeviceCollectionState>) -> status::NoContent {
    devices.lock().unwrap().remove(&uuid);
    status::NoContent
}

#[cfg(test)]
mod tests;
