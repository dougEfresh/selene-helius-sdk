use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use serde_json::{Number, Value};

pub(crate) fn deserialize_u32_from_null<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
  D: Deserializer<'de>,
{
  let opt = Option::<u32>::deserialize(deserializer)?;
  Ok(opt.unwrap_or_default())
}

pub(crate) fn deserialize_str_to_number<'de, D>(deserializer: D) -> Result<Number, D::Error>
where
  D: Deserializer<'de>,
{
  let v: Value = Deserialize::deserialize(deserializer)?;
  match v {
    Value::String(s) => s.parse::<Number>().map_err(SerdeError::custom),
    Value::Number(n) => Ok(n),
    _ => Err(SerdeError::custom("Expected a string or number")),
  }
}

#[cfg(test)]
mod tests {
  use crate::util::deserialize_str_to_number;
  use serde::Deserialize;
  use serde_json::Number;

  #[derive(Deserialize, Debug)]
  struct Blah {
    #[serde(deserialize_with = "deserialize_str_to_number")]
    num_str: Number,
  }

  #[test]
  fn str_to_number() -> color_eyre::Result<()> {
    let blah: Blah = serde_json::from_str(r#"{"num_str": 2 }"#)?;
    assert_eq!(blah.num_str.as_i64().unwrap_or(0), 2);
    Ok(())
  }
}
