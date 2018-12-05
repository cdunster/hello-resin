use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Device {
    name: String,
    setpoint: f64,
    zone_uuid: Option<Uuid>,
}

impl Device {
    pub fn new(name: String, zone_uuid: Option<Uuid>) -> Device {
        Device {
            name,
            zone_uuid,
            setpoint: 16.0,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_zone_uuid(&mut self, zone_uuid: Option<Uuid>) {
        self.zone_uuid = zone_uuid;
    }

    pub fn set_setpoint(&mut self, setpoint: f64) {
        self.setpoint = setpoint;
    }
}

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

    pub fn add(&mut self, uuid: Uuid, device: Device) {
        self.devices.insert(uuid, device);
    }

    pub fn get(&self, uuid: &Uuid) -> Option<&Device> {
        self.devices.get(uuid)
    }

    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Device> {
        self.devices.get_mut(uuid)
    }

    pub fn get_all_with_zone(&self, zone_uuid: Uuid) -> Option<DeviceCollection> {
        let mut devices = self.devices.clone();
        devices.retain(|_, device| device.zone_uuid == Some(zone_uuid));
        if devices.is_empty() {
            None
        } else {
            Some(DeviceCollection { devices })
        }
    }
}
