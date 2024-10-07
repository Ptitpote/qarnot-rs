use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::fmt;
use std::marker::PhantomData;

use std::collections::HashMap;

/// Wrapper struct over HashMap to manipulate constants in a more convinent way
/// It make constants a single HashMap and handle the representation on the
/// Qarnot API with custom Serialize/Deserialize implementation.
#[derive(Clone, Default, Debug)]
pub struct Constants(pub HashMap<String, String>);

impl Constants {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        self.0.insert(key.to_owned(), value.to_owned())
    }
}

impl Serialize for Constants {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            let mut constant = HashMap::new();
            constant.insert("key", k);
            constant.insert("value", v);
            seq.serialize_element(&constant)?;
        }
        seq.end()
    }
}

struct ConstantVisitor {
    marker: PhantomData<fn() -> Constants>,
}

impl ConstantVisitor {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for ConstantVisitor {
    // The type that our Visitor is going to produce.
    type Value = Constants;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("List of constants or a single constant")
    }

    // Deserialize Constants from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry::<&str, &str>()? {
            if key == "key" {
                let keyname = value;
                if let Some((key, value)) = access.next_entry::<&str, &str>()? {
                    if key == "value" {
                        map.insert(keyname.to_owned(), value.to_owned());
                    }
                }
            }
        }

        Ok(Constants(map))
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0) * 2);

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some(elt) = access.next_element::<HashMap<&str, &str>>()? {
            let mut key = "";
            let mut value = "";
            for (k, v) in elt {
                if k == "key" {
                    key = v;
                    continue;
                }
                if k == "value" {
                    value = v;
                }
            }
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Constants(map))
    }
}

// This is the trait that informs Serde how to deserialize Constants.
impl<'de> Deserialize<'de> for Constants {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of Constants.
        deserializer.deserialize_seq(ConstantVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_constants() {
        let example_str = r#"
        [
            {
              "key": "BLEND_FILE",
              "value": "final.blend"
            },
            {
              "key": "DOCKER_USER",
              "value": "mysuperuser"
            }
        ]
        "#;
        let constants = serde_json::from_str::<Constants>(example_str);
        assert!(constants.is_ok());
        let constants = constants.unwrap();
        for (key, value) in constants.0 {
            if key == "BLEND_FILE" {
                assert!(value == "final.blend");
            }
            if key == "DOCKER_USER" {
                assert!(value == "mysuperuser");
            }
        }
    }

    #[test]
    fn serialize_constants() {
        let example_str = r#"[{"key":"BLEND_FILE","value":"final.blend"}]"#;
        // The custom serializer MAY not preserve the key/value order.
        // Not an issue in itself, but if not checked the test might fail.
        let example_str_flipped = r#"[{"value":"final.blend","key":"BLEND_FILE"}]"#;
        let mut constants = Constants::new();
        constants.insert("BLEND_FILE", "final.blend");

        let res = serde_json::to_string(&constants);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(
            res == example_str || res == example_str_flipped,
            "serialized constant: {}",
            res
        );
    }
}
