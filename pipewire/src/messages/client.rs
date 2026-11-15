use std::collections::BTreeMap;

use spa::{EncodePod, PodBuf};

use crate::PwMethod;

#[derive(Debug)]
pub struct UpdateProperties {
    pub properties: BTreeMap<String, String>,
}

impl EncodePod for UpdateProperties {
    fn encode(&self, dst: &mut PodBuf) {
        let n_items = u32::try_from(self.properties.len()).expect("int overflow");
        dst.put_struct(|dst| {
            dst.put_struct(|dst| {
                dst.put_int(n_items as i32);
                for (key, val) in self.properties.iter() {
                    dst.put_string(key);
                    dst.put_string(val);
                }
            });
        })
    }
}

impl PwMethod for UpdateProperties {
    fn id(&self) -> u32 {
        1
    }
    fn opcode(&self) -> u8 {
        2
    }
}
