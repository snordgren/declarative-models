use serde::{Deserialize, Serialize};

use crate::{GeometryBuffer, Vector3};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cube {
  #[serde(default)]
  pub position: Vector3,
  #[serde(default)]
  pub size: Vector3,
  #[serde(default)]
  pub rotation: Option<Vector3>,
  #[serde(default)]
  pub offsets: Option<CubeVertexOffset>,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct CubeVertexOffset {
  #[serde(default)]
  pub v000: Vector3,
  #[serde(default)]
  pub v001: Vector3,
  #[serde(default)]
  pub v010: Vector3,
  #[serde(default)]
  pub v011: Vector3,
  #[serde(default)]
  pub v100: Vector3,
  #[serde(default)]
  pub v101: Vector3,
  #[serde(default)]
  pub v110: Vector3,
  #[serde(default)]
  pub v111: Vector3,
}

impl Cube {
  pub fn generate_geometry(&self) -> GeometryBuffer {
    let mut buf = GeometryBuffer::new();

    let min = Vector3::new(-0.5, -0.5, -0.5);
    let max = min + Vector3::ONE;

    let offset = self.offsets.unwrap_or_default();
    let os = Vector3::ONE / self.size;

    let p000 = buf.vertex(min + offset.v000);
    let p001 = buf.vertex(Vector3::new(min.x, min.y, max.z) + offset.v001 * os);
    let p010 = buf.vertex(Vector3::new(min.x, max.y, min.z) + offset.v010 * os);
    let p011 = buf.vertex(Vector3::new(min.x, max.y, max.z) + offset.v011 * os);
    let p100 = buf.vertex(Vector3::new(max.x, min.y, min.z) + offset.v100 * os);
    let p101 = buf.vertex(Vector3::new(max.x, min.y, max.z) + offset.v101 * os);
    let p110 = buf.vertex(Vector3::new(max.x, max.y, min.z) + offset.v110 * os);
    let p111 = buf.vertex(Vector3::new(max.x, max.y, max.z) + offset.v111 * os);

    // back
    buf.triangle(p000, p010, p100);
    buf.triangle(p010, p110, p100);

    // front
    buf.triangle(p001, p101, p011);
    buf.triangle(p011, p101, p111);

    // bottom
    buf.triangle(p000, p100, p001);
    buf.triangle(p100, p101, p001);

    // top
    buf.triangle(p010, p011, p110);
    buf.triangle(p110, p011, p111);

    // left
    buf.triangle(p000, p001, p010);
    buf.triangle(p001, p011, p010);

    // right
    buf.triangle(p100, p110, p101);
    buf.triangle(p101, p110, p111);

    buf
  }
}
