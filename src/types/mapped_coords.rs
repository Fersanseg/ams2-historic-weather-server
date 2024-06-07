use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MappedCoords<'a> {
  pub name: &'a str,
  pub latitude: f64,
  pub longitude: f64
}