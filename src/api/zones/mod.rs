use rocket::response::status;
use rocket::{Rocket, State};
use rocket_contrib::{Json, UUID};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct Zone {
    name: String,
}

type ZoneCollectionState = Mutex<ZoneCollection>;

#[derive(Serialize)]
pub struct ZoneCollection {
    zones: HashMap<Uuid, Zone>,
}

impl ZoneCollection {
    pub fn new() -> ZoneCollection {
        ZoneCollection {
            zones: HashMap::new(),
        }
    }

    fn add(&mut self, zone: Zone) -> Entry<Uuid, Zone> {
        let uuid = Uuid::new_v4();
        self.zones.insert(uuid, zone);

        self.zones.entry(uuid)
    }

    fn get(&self, uuid: &Uuid) -> Option<&Zone> {
        self.zones.get(uuid)
    }

    fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Zone> {
        self.zones.get_mut(uuid)
    }

    fn remove(&mut self, uuid: &Uuid) {
        self.zones.remove(uuid);
    }
}

pub fn mount(rocket: Rocket, zones: ZoneCollection) -> Rocket {
    rocket
        .mount(
            "/zones",
            routes![
                get_zones,
                post_zones,
                get_zone_from_uuid,
                patch_zone_from_uuid,
                delete_zone_from_uuid
            ],
        ).manage(ZoneCollectionState::new(zones))
}

#[get("/", format = "application/json")]
fn get_zones(zones: State<ZoneCollectionState>) -> Json {
    Json(json!(zones.inner()))
}

#[post("/", format = "application/json", data = "<zone>")]
fn post_zones(zone: Json<Zone>, zones: State<ZoneCollectionState>) -> status::Created<Json<Zone>> {
    let mut zones = zones.lock().unwrap();
    let zone_entry = zones.add(zone.clone());

    status::Created(format!("/zones/{}", zone_entry.key()), Some(zone))
}

#[get("/<uuid>", format = "application/json")]
fn get_zone_from_uuid(uuid: UUID, zones: State<ZoneCollectionState>) -> Option<Json<Zone>> {
    if let Some(zone) = zones.lock().unwrap().get(&uuid.into_inner()) {
        Some(Json(zone.clone()))
    } else {
        None
    }
}

#[patch(
    "/<uuid>",
    format = "application/json",
    data = "<patch_json>"
)]
fn patch_zone_from_uuid(
    uuid: UUID,
    patch_json: Json,
    zones: State<ZoneCollectionState>,
) -> Option<Json<Zone>> {
    if let Some(zone) = zones.lock().unwrap().get_mut(&uuid.into_inner()) {
        if let Some(patch_name) = patch_json["name"].as_str() {
            zone.name = patch_name.to_string().clone();
        }
        Some(Json(zone.clone()))
    } else {
        None
    }
}

#[delete("/<uuid>", format = "application/json")]
fn delete_zone_from_uuid(uuid: UUID, zones: State<ZoneCollectionState>) -> status::NoContent {
    zones.lock().unwrap().remove(&uuid);
    status::NoContent
}

#[cfg(test)]
mod tests;
