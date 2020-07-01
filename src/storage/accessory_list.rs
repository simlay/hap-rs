use std::sync::Arc;

use erased_serde::{self, serialize_trait_object};
use futures::lock::Mutex;
use log::debug;
use serde_json::json;

use crate::{
    accessory::HapAccessory,
    characteristic::Perm,
    pointer,
    transport::http::{ReadResponseObject, Status, WriteObject, WriteResponseObject},
    Error,
    Result,
};

/// `AccessoryList` is a wrapper type holding a list of Accessories.
pub struct AccessoryList {
    pub accessories: Vec<pointer::AccessoryListMember>,
    event_emitter: pointer::EventEmitter,
    id_count: u64, // TODO: create the IDs elsewhere; e.g. inside of accessories when adding them
}

impl AccessoryList {
    /// Creates a new `AccessoryList`.
    pub fn new(event_emitter: pointer::EventEmitter) -> AccessoryList {
        AccessoryList {
            accessories: Vec::new(),
            event_emitter,
            id_count: 1,
        }
    }

    /// Adds an Accessory to the `AccessoryList` and returns a pointer to the added Accessory.
    pub async fn add_accessory(
        &mut self,
        accessory: Box<dyn AccessoryListMember + Send + Sync>,
    ) -> Result<pointer::AccessoryListMember> {
        let mut a = accessory;
        a.set_id(self.id_count);
        a.init_iids(self.id_count, self.event_emitter.clone())?;
        let a_ptr = Arc::new(Mutex::new(a));
        self.accessories.push(a_ptr.clone());
        self.id_count += 1;

        Ok(a_ptr)
    }

    /// Takes a pointer to an Accessory and removes the Accessory from the `AccessoryList`.
    pub async fn remove_accessory(&mut self, accessory: &pointer::AccessoryListMember) -> Result<()> {
        let accessory = accessory.lock().await;
        let mut remove = None;

        for (i, a) in self.accessories.iter().enumerate() {
            if a.lock().await.get_id() == accessory.get_id() {
                remove = Some(i);
                break;
            }
        }

        if let Some(i) = remove {
            self.accessories.remove(i);

            return Ok(());
        }

        Err(Error::from_str("couldn't find the Accessory to remove"))
    }

    pub(crate) async fn read_characteristic(
        &self,
        aid: u64,
        iid: u64,
        meta: bool,
        perms: bool,
        hap_type: bool,
        ev: bool,
    ) -> Result<ReadResponseObject> {
        let mut result_object = ReadResponseObject {
            iid,
            aid,
            hap_type: None,
            format: None,
            perms: None,
            ev: None,
            value: None,
            unit: None,
            max_value: None,
            min_value: None,
            step_value: None,
            max_len: None,
            status: Some(0),
        };

        'l: for accessory in self.accessories.iter() {
            let mut a = accessory.lock().await;
            if a.get_id() == aid {
                for service in a.get_mut_services() {
                    for characteristic in service.get_mut_characteristics() {
                        if characteristic.get_id() == iid {
                            let characteristic_perms = characteristic.get_perms();
                            if characteristic_perms.contains(&Perm::PairedRead) {
                                result_object.value = Some(characteristic.get_value().await?);
                                if meta {
                                    result_object.format = Some(characteristic.get_format());
                                    result_object.unit = characteristic.get_unit();
                                    result_object.max_value = characteristic.get_max_value();
                                    result_object.min_value = characteristic.get_min_value();
                                    result_object.step_value = characteristic.get_step_value();
                                    result_object.max_len = characteristic.get_max_len();
                                }
                                if perms {
                                    result_object.perms = Some(characteristic_perms);
                                }
                                if hap_type {
                                    result_object.hap_type = Some(characteristic.get_type());
                                }
                                if ev {
                                    result_object.ev = characteristic.get_event_notifications();
                                }
                            } else {
                                result_object.status = Some(Status::WriteOnlyCharacteristic as i32);
                            }
                            break 'l;
                        }
                    }
                }
            }
        }

        Ok(result_object)
    }

    pub(crate) async fn write_characteristic(
        &mut self,
        write_object: WriteObject,
        event_subscriptions: &pointer::EventSubscriptions,
    ) -> Result<WriteResponseObject> {
        let mut result_object = WriteResponseObject {
            aid: write_object.aid,
            iid: write_object.iid,
            status: 0,
        };

        'l: for accessory in self.accessories.iter_mut() {
            let mut a = accessory.lock().await;
            if a.get_id() == write_object.aid {
                for service in a.get_mut_services() {
                    for characteristic in service.get_mut_characteristics() {
                        if characteristic.get_id() == write_object.iid {
                            let characteristic_perms = characteristic.get_perms();
                            if let Some(ev) = write_object.ev {
                                if characteristic_perms.contains(&Perm::Events) {
                                    characteristic.set_event_notifications(Some(ev));
                                    let subscription = (write_object.aid, write_object.iid);
                                    let mut es = event_subscriptions.lock().await;
                                    let pos = es.iter().position(|&s| s == subscription);
                                    match (ev, pos) {
                                        (true, None) => {
                                            es.push(subscription);
                                        },
                                        (false, Some(p)) => {
                                            es.remove(p);
                                        },
                                        _ => {},
                                    }
                                } else {
                                    result_object.status = Status::NotificationNotSupported as i32;
                                }
                            }
                            if let Some(value) = write_object.value {
                                if characteristic_perms.contains(&Perm::PairedWrite) {
                                    characteristic.set_value(value).await?;
                                } else {
                                    result_object.status = Status::ReadOnlyCharacteristic as i32;
                                }
                            }
                            break 'l;
                        }
                    }
                }
            }
        }

        Ok(result_object)
    }

    pub(crate) async fn as_serialized_json(&self) -> Result<Vec<u8>> {
        let mut accessory_values = Vec::new();
        for accessory in &self.accessories {
            let a = accessory.lock().await;
            accessory_values.push(serde_json::to_value(&*a)?);
        }

        debug!("accessory list JSON: {}", &json!({ "accessories": accessory_values }));

        Ok(serde_json::to_vec(&json!({ "accessories": accessory_values }))?)
    }
}

/// `AccessoryListMember` is implemented by members of an `AccessoryList`.
pub trait AccessoryListMember: HapAccessory + erased_serde::Serialize {}

impl<T: HapAccessory + erased_serde::Serialize> AccessoryListMember for T {}

serialize_trait_object!(AccessoryListMember);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_serialization() {} // TODO: test it
}
