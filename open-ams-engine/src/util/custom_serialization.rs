use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use serde::de::{self, Deserializer, MapAccess, Visitor};
use std::fmt;

pub(crate) fn custom_deserialize_map_or_seq<'de, D, T>(
    deserializer: D,
) -> Result<HashMap<String, T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    struct MapOrSeqVisitor<T> {
        marker: std::marker::PhantomData<T>,
    }

    impl<'de, T> Visitor<'de> for MapOrSeqVisitor<T>
    where
        T: Deserialize<'de> + Default,
    {
        type Value = HashMap<String, T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map or a list of strings")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some((key, value)) = access.next_entry()? {
                map.insert(key, value);
            }
            Ok(map)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some(key) = seq.next_element::<String>()? {
                map.insert(key, T::default());
            }
            Ok(map)
        }
    }

    deserializer.deserialize_any(MapOrSeqVisitor {
        marker: std::marker::PhantomData,
    })
}
