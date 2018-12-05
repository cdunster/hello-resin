use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Zone {
    name: String,
    setpoint: f64,
}

impl Zone {
    pub fn new(name: String) -> Zone {
        Zone { name, setpoint: 16.0 }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_setpoint(&mut self, setpoint: f64) {
        self.setpoint = setpoint;
    }
}

#[derive(Serialize)]
pub struct ZoneCollection {
    zones: HashMap<Uuid, Zone>,
}

impl ZoneCollection {
    pub fn new() -> ZoneCollection {
        ZoneCollection { zones: HashMap::new() }
    }

    pub fn add(&mut self, uuid: Uuid, zone: Zone) {
        self.zones.insert(uuid, zone);
    }

    pub fn get(&self, uuid: &Uuid) -> Option<&Zone> {
        self.zones.get(uuid)
    }

    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Zone> {
        self.zones.get_mut(uuid)
    }

    pub fn remove(&mut self, uuid: &Uuid) {
        self.zones.remove(uuid);
    }
}
