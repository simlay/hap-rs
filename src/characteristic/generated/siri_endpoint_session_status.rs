// this file is auto-generated by hap-codegen

use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;

use crate::{
    characteristic::{
        AsyncCharacteristicCallbacks,
        Characteristic,
        CharacteristicCallbacks,
        Format,
        HapCharacteristic,
        HapCharacteristicSetup,
        HapType,
        OnReadFn,
        OnReadFuture,
        OnUpdateFn,
        OnUpdateFuture,
        Perm,
        Unit,
    },
    pointer,
    Error,
    Result,
};

// TODO - re-check MaximumDataLength
/// Siri Endpoint Session Status characteristic.
#[derive(Debug, Default, Serialize)]
pub struct SiriEndpointSessionStatusCharacteristic(Characteristic<Vec<u8>>);

impl SiriEndpointSessionStatusCharacteristic {
    /// Creates a new Siri Endpoint Session Status characteristic.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        #[allow(unused_mut)]
        let mut c = Self(Characteristic::<Vec<u8>> {
            id,
            accessory_id,
            hap_type: HapType::SiriEndpointSessionStatus,
            format: Format::Tlv8,
            perms: vec![
				Perm::Events,
				Perm::PairedRead,
            ],
            ..Default::default()
        });

        if let Some(ref min_value) = &c.0.min_value {
            c.0.value = min_value.clone();
        } else if let Some(ref valid_values) = &c.0.valid_values {
            if valid_values.len() > 0 {
                c.0.value = valid_values[0].clone();
            }
        }

        c
    }
}

#[async_trait]
impl HapCharacteristic for SiriEndpointSessionStatusCharacteristic {
    fn get_id(&self) -> u64 { self.0.get_id() }

    fn set_id(&mut self, id: u64) { self.0.set_id(id) }

    fn get_type(&self) -> HapType { self.0.get_type() }

    fn set_type(&mut self, hap_type: HapType) { self.0.set_type(hap_type) }

    fn get_format(&self) -> Format { self.0.get_format() }

    fn set_format(&mut self, format: Format) { self.0.set_format(format) }

    fn get_perms(&self) -> Vec<Perm> { self.0.get_perms() }

    fn set_perms(&mut self, perms: Vec<Perm>) { self.0.set_perms(perms) }

    fn get_description(&self) -> Option<String> { self.0.get_description() }

    fn set_description(&mut self, description: Option<String>) { self.0.set_description(description) }

    fn get_event_notifications(&self) -> Option<bool> { self.0.get_event_notifications() }

    fn set_event_notifications(&mut self, event_notifications: Option<bool>) {
        self.0.set_event_notifications(event_notifications)
    }

    async fn get_value(&mut self) -> Result<serde_json::Value> {
        let value = self.0.get_value().await?;
        Ok(json!(value))
    }

    async fn set_value(&mut self, value: serde_json::Value) -> Result<()> {
        let v;
        // for whatever reason, the controller is setting boolean values either as a boolean or as an integer
        if self.0.format == Format::Bool && value.is_number() {
            let num_v: u8 = serde_json::from_value(value)?;
            if num_v == 0 {
                v = serde_json::from_value(json!(false))?;
            } else if num_v == 1 {
                v = serde_json::from_value(json!(true))?;
            } else {
                return Err(Error::InvalidValue(self.get_format()));
            }
        } else {
            v = serde_json::from_value(value).map_err(|_| Error::InvalidValue(self.get_format()))?;
        }
        self.0.set_value(v).await
    }

    fn get_unit(&self) -> Option<Unit> { self.0.get_unit() }

    fn set_unit(&mut self, unit: Option<Unit>) { self.0.set_unit(unit) }

    fn get_max_value(&self) -> Option<serde_json::Value> { self.0.get_max_value().map(|v| json!(v)) }

    fn set_max_value(&mut self, max_value: Option<serde_json::Value>) -> Result<()> {
        self.0.set_max_value(match max_value {
            Some(v) => Some(serde_json::from_value(v).map_err(|_| Error::InvalidValue(self.get_format()))?),
            None => None,
        });

        Ok(())
    }

    fn get_min_value(&self) -> Option<serde_json::Value> { self.0.get_min_value().map(|v| json!(v)) }

    fn set_min_value(&mut self, min_value: Option<serde_json::Value>) -> Result<()> {
        self.0.set_min_value(match min_value {
            Some(v) => Some(serde_json::from_value(v).map_err(|_| Error::InvalidValue(self.get_format()))?),
            None => None,
        });

        Ok(())
    }

    fn get_step_value(&self) -> Option<serde_json::Value> { self.0.get_step_value().map(|v| json!(v)) }

    fn set_step_value(&mut self, step_value: Option<serde_json::Value>) -> Result<()> {
        self.0.set_step_value(match step_value {
            Some(v) => Some(serde_json::from_value(v).map_err(|_| Error::InvalidValue(self.get_format()))?),
            None => None,
        });

        Ok(())
    }

    fn get_max_len(&self) -> Option<u16> { self.0.get_max_len() }

    fn set_max_len(&mut self, max_len: Option<u16>) { self.0.set_max_len(max_len) }

    fn get_max_data_len(&self) -> Option<u32> { self.0.get_max_data_len() }

    fn set_max_data_len(&mut self, max_data_len: Option<u32>) { self.0.set_max_data_len(max_data_len) }

    fn get_valid_values(&self) -> Option<Vec<serde_json::Value>> {
        self.0
            .get_valid_values()
            .map(|v| v.into_iter().map(|v| json!(v)).collect())
    }

    fn set_valid_values(&mut self, valid_values: Option<Vec<serde_json::Value>>) -> Result<()> {
        self.0.set_valid_values(match valid_values {
            Some(v) => Some(
                v.into_iter()
                    .map(|v| serde_json::from_value(v).map_err(|_| Error::InvalidValue(self.get_format())))
                    .collect::<Result<Vec<Vec<u8>>>>()?,
            ),
            None => None,
        });

        Ok(())
    }

    fn get_valid_values_range(&self) -> Option<[serde_json::Value; 2]> {
        self.0.get_valid_values_range().map(|v| [json!(v[0]), json!(v[1])])
    }

    fn set_valid_values_range(&mut self, valid_values_range: Option<[serde_json::Value; 2]>) -> Result<()> {
        self.0.set_valid_values_range(match valid_values_range {
            Some([start, end]) => Some(Result::<[Vec<u8>; 2]>::Ok([
                serde_json::from_value(start).map_err(|_| Error::InvalidValue(self.get_format()))?,
                serde_json::from_value(end).map_err(|_| Error::InvalidValue(self.get_format()))?,
            ])?),
            None => None,
        });

        Ok(())
    }

    fn get_ttl(&self) -> Option<u64> { self.0.get_ttl() }

    fn set_ttl(&mut self, ttl: Option<u64>) { self.0.set_ttl(ttl) }

    fn get_pid(&self) -> Option<u64> { self.0.get_pid() }

    fn set_pid(&mut self, pid: Option<u64>) { self.0.set_pid(pid) }
}

impl HapCharacteristicSetup for SiriEndpointSessionStatusCharacteristic {
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        self.0.set_event_emitter(event_emitter)
    }
}

impl CharacteristicCallbacks<Vec<u8>> for SiriEndpointSessionStatusCharacteristic {
    fn on_read(&mut self, f: Option<impl OnReadFn<Vec<u8>>>) { self.0.on_read(f) }

    fn on_update(&mut self, f: Option<impl OnUpdateFn<Vec<u8>>>) { self.0.on_update(f) }
}

impl AsyncCharacteristicCallbacks<Vec<u8>> for SiriEndpointSessionStatusCharacteristic {
    fn on_read_async(&mut self, f: Option<impl OnReadFuture<Vec<u8>>>) { self.0.on_read_async(f) }

    fn on_update_async(&mut self, f: Option<impl OnUpdateFuture<Vec<u8>>>) { self.0.on_update_async(f) }
}