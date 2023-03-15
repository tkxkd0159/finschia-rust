use anyhow::Result;
use prost::{DecodeError, Message};
use prost_types::Any;

pub fn to_any<T>(msg: &T, type_url: &str) -> Any
where
    T: Message + Sized,
{
    Any {
        type_url: type_url.to_string(),
        value: msg.encode_to_vec(),
    }
}

pub fn from_any<T>(msg: &Any) -> Result<T>
where
    T: Message + Sized + Default,
{
    let decoded_any = T::decode(msg.value.as_slice())?;
    Ok(decoded_any)
}
