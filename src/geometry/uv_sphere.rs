use serde::{Deserialize, Serialize};

use crate::{GeometryBuffer, Vector3, GenerateGeometry};
use genmesh::generators::{IndexedPolygon, SharedVertex};
use genmesh::Polygon;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UvSphere {
  #[serde(default = "default_u")]
  pub u: u32,
  #[serde(default = "default_v")]
  pub v: u32,
  #[serde(default)]
  pub size: Option<Vector3>,
  #[serde(default)]
  pub rotation: Option<Vector3>,
  #[serde(default)]
  pub position: Option<Vector3>,
}

fn default_u() -> u32 {
  8
}

fn default_v() -> u32 {
  8
}

impl GenerateGeometry for UvSphere {
  fn generate_geometry(&self) -> GeometryBuffer {
    let mut buf = GeometryBuffer::new();

    let mesh = genmesh::generators::SphereUv::new(
      self.u as usize, self.v as usize);

    for vertex in mesh.shared_vertex_iter() {
      buf.vertex(Vector3::new(
        vertex.pos.x,
        vertex.pos.y,
        vertex.pos.z,
      ));
    }

    for polygon in mesh.indexed_polygon_iter() {
      match polygon {
        Polygon::PolyTri(triangle) => {
          buf.triangle(
            triangle.x as u16,
            triangle.y as u16,
            triangle.z as u16,
          );
        }
        Polygon::PolyQuad(quad) => {
          buf.triangle(quad.x as u16, quad.y as u16, quad.z as u16);
          buf.triangle(quad.x as u16, quad.z as u16, quad.w as u16);
        }
      }
    }

    buf.scale(Vector3::new(0.5, 0.5, 0.5));
    buf.rotate(Vector3::new(0.0, 0.0, 90.0));
    buf
  }
}
