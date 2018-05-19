extern crate serde;
extern crate serde_json;
extern crate serde_xml_rs;

pub trait SchemaObject<'a> : serde::Serialize + serde::Deserialize<'a> {
    fn deserialize_me(bytes: &'a [u8], is_good: bool) -> Self {
        if is_good { 
            serde_json::from_slice(bytes).unwrap()
        } else {
            serde_xml_rs::deserialize(bytes).unwrap()
        }
    }
}
