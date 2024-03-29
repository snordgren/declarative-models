use genmesh::generators::{IndexedPolygon, SharedVertex};
use serde::{Deserialize, Serialize};

use crate::{GenerateGeometry, GeometryBuffer, Vector3};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cone {
  #[serde(default = "default_divides")]
  pub divides: u32,
  #[serde(default)]
  pub size: Option<Vector3>,
  #[serde(default)]
  pub rotation: Option<Vector3>,
  #[serde(default)]
  pub position: Option<Vector3>,
}

fn default_divides() -> u32 {
  1
}

impl GenerateGeometry for Cone {
  fn generate_geometry(&self) -> GeometryBuffer {
    let mut buf = GeometryBuffer::new();

    let cone = genmesh::generators::Cone::new(
      self.divides as usize);

    for vertex in cone.shared_vertex_iter() {
      buf.vertex(Vector3::new(
        vertex.pos.x,
        vertex.pos.y,
        vertex.pos.z,
      ));
    }

    for triangle in cone.indexed_polygon_iter() {
      buf.triangle(
        triangle.x as u16,
        triangle.y as u16,
        triangle.z as u16,
      );
    }

    buf.scale(Vector3::new(0.5, 0.5, 0.5));
    buf.rotate(Vector3::new(0.0, 0.0, -90.0));
    buf
  }
}
