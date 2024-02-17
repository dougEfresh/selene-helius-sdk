use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use serde_json::{Number, Value};
use tracing_subscriber::EnvFilter;

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

#[allow(dead_code, clippy::missing_errors_doc)]
pub fn init_tracing() -> color_eyre::Result<()> {
  color_eyre::install()?;
  let filter = EnvFilter::from_default_env();
  let subscriber = tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).with_target(true).finish();
  tracing::subscriber::set_global_default(subscriber)?;
  Ok(())
}
