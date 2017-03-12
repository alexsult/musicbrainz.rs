use serde::{Serialize, Serializer, Deserialize, Deserializer};
use uuid::Uuid;

pub fn uuid_from_string<D>(deserializer: D) -> Result<Uuid, D::Error>
    where D: Deserializer {
    let uuid_string: String = Deserialize::deserialize(deserializer).unwrap();
    if uuid_string.len() > 0 { 
        Ok(Uuid::parse_str(uuid_string.as_str()).unwrap()) 
    }
    else {
        Ok(Uuid::nil())
    }
}

pub fn string_from_uuid<S>(uuid_elem: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    uuid_elem.hyphenated().to_string().serialize(serializer)
}

pub fn deserialize_bool<D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer {
    let bool_return: bool = match Deserialize::deserialize(deserializer) {
        Ok(x) => x,
        Err(e) => false
    };

    Ok(bool_return)
}
