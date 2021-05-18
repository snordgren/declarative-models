use serde::{Deserialize, Serialize};
use crate::{GeometryBuffer, Geometry, Vector3};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deform {
  #[serde(default)]
  pub seed: u64,
  #[serde(default = "Vector3::minus_one")]
  pub min: Vector3,
  #[serde(default = "Vector3::one")]
  pub max: Vector3,
  pub geometry: Geometry,
}

impl Deform {
  pub fn generate_geometry(&self) -> GeometryBuffer {
    let mut buf = self.geometry.generate_vertices();
    buf.deform(self.min, self.max, Vector3::ONE, self.seed);
    buf
  }
}
