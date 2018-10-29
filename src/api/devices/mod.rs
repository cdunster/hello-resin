use rocket::{Rocket, State};
use rocket_contrib::{Json, UUID};
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

    fn get(&self, uuid: &Uuid) -> Option<&Device> {
        self.devices.get(uuid)
    }

    fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Device> {
        self.devices.get_mut(uuid)
    }
}

pub fn mount(rocket: Rocket, devices: DeviceCollection) -> Rocket {
    rocket
        .mount(
            "/devices",
            routes![get_devices, get_device_from_uuid, patch_device_from_uuid,],
        ).manage(DeviceCollectionState::new(devices))
}

#[get("/", format = "application/json")]
fn get_devices(devices: State<DeviceCollectionState>) -> Json {
    Json(json!(devices.inner()))
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

#[cfg(test)]
mod tests;
