use rocket::response::status;
use rocket::{Rocket, State};
use rocket_contrib::{Json, UUID};
use std::sync::Mutex;
use uuid::Uuid;
use zone::{Zone, ZoneCollection};

type ZoneCollectionState = Mutex<ZoneCollection>;

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
    let uuid = Uuid::new_v4();
    zones.add(uuid, zone.clone());

    status::Created(format!("/zones/{}", uuid), Some(zone))
}

#[get("/<uuid>", format = "application/json")]
fn get_zone_from_uuid(uuid: UUID, zones: State<ZoneCollectionState>) -> Option<Json<Zone>> {
    if let Some(zone) = zones.lock().unwrap().get(&uuid.into_inner()) {
        Some(Json(zone.clone()))
    } else {
        None
    }
}

fn patch_zone_with_json(zone: &mut Zone, patch_json: &Json) {
    let patch_json = patch_json.as_object().unwrap();

    if patch_json.contains_key("name") {
        if let Some(patch_name) = patch_json["name"].as_str() {
            zone.set_name(patch_name.to_string());
        }
    }

    if patch_json.contains_key("setpoint") {
        if let Some(patch_setpoint) = patch_json["setpoint"].as_f64() {
            zone.set_setpoint(patch_setpoint);
        }
    }
}

#[patch("/<uuid>", format = "application/json", data = "<patch_json>")]
fn patch_zone_from_uuid(uuid: UUID, patch_json: Json, zones: State<ZoneCollectionState>) -> Option<Json<Zone>> {
    if let Some(zone) = zones.lock().unwrap().get_mut(&uuid.into_inner()) {
        patch_zone_with_json(zone, &patch_json);
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
