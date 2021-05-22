use serde::{Deserialize, Serialize};

use crate::{GeometryBuffer, Vector3, GenerateGeometry};
use genmesh::generators::{IndexedPolygon, SharedVertex};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Icosphere {
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
  0
}

impl GenerateGeometry for Icosphere {
  fn generate_geometry(&self) -> GeometryBuffer {
    let mut buf = GeometryBuffer::new();
    let icosphere = genmesh::generators::IcoSphere::subdivide(
      self.divides as usize);

    for vertex in icosphere.shared_vertex_iter() {
      buf.vertex(Vector3::new(
        vertex.pos.x,
        vertex.pos.y,
        vertex.pos.z,
      ));
    }

    for triangle in icosphere.indexed_polygon_iter() {
      buf.triangle(
        triangle.x as u16,
        triangle.y as u16,
        triangle.z as u16,
      );
    }

    buf.scale(Vector3::new(0.5, 0.5, 0.5));
    buf.rotate(Vector3::new(0.0, 0.0, 90.0));
    buf
  }
}
