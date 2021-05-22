use serde::{Deserialize, Serialize};

pub use cone::*;
pub use cube::*;
pub use cylinder::*;
pub use deform::*;
pub use icosphere::*;
pub use uv_sphere::*;

use crate::{GeometryBuffer, Vector3};

mod cone;
mod cube;
mod cylinder;
mod deform;
mod icosphere;
mod uv_sphere;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Geometry {
  Cone(Cone),
  Cube(Cube),
  Cylinder(Cylinder),
  Deform(Box<Deform>),
  Icosphere(Icosphere),
  Plane(Plane),
  Triangle(Triangle),
  UvSphere(UvSphere),
}

impl GenerateGeometry for Geometry {
  fn generate_geometry(&self) -> GeometryBuffer {
    let rotation;
    let scale;
    let translation;

    let mut buf = GeometryBuffer::new();

    match self {
      Geometry::Cone(c) => {
        buf = c.generate_geometry();

        rotation = c.rotation;
        scale = c.size;
        translation = c.position;
      }
      Geometry::Cube(b) => {
        buf = b.generate_geometry();

        rotation = b.rotation;
        scale = Some(b.size);
        translation = Some(b.position);
      }
      Geometry::Cylinder(c) => {
        buf = c.generate_geometry();

        rotation = c.rotation;
        scale = Some(c.size);
        translation = Some(c.position);
      }
      Geometry::Deform(i) => {
        buf = i.generate_geometry();

        rotation = None;
        scale = None;
        translation = None;
      }
      Geometry::Icosphere(i) => {
        buf = i.generate_geometry();

        rotation = i.rotation;
        scale = i.size;
        translation = i.position;
      }
      Geometry::Plane(p) => {
        let min = p.position - p.size / Vector3::new(2.0, 2.0, 2.0);
        let max = min + p.size;

        let v00 = buf.vertex(Vector3::new(min.x, 0.0, min.y));
        let v01 = buf.vertex(Vector3::new(min.x, 0.0, max.y));
        let v10 = buf.vertex(Vector3::new(max.x, 0.0, min.y));
        let v11 = buf.vertex(Vector3::new(max.x, 0.0, max.y));

        buf.triangle(v00, v10, v01);
        buf.triangle(v01, v10, v11);

        rotation = p.rotation;
        scale = None;
        translation = None;
      }
      Geometry::Triangle(triangle) => {
        let v0 = buf.vertex(triangle.points[0]);
        let v1 = buf.vertex(triangle.points[1]);
        let v2 = buf.vertex(triangle.points[2]);

        buf.triangle(v0, v1, v2);

        rotation = triangle.rotation;
        scale = None;
        translation = None;
      }
      Geometry::UvSphere(i) => {
        buf = i.generate_geometry();

        rotation = i.rotation;
        scale = i.size;
        translation = i.position;
      }
    }

    if let Some(matrix) = scale {
      buf.scale(matrix);
    }

    if let Some(rot) = rotation {
      buf.rotate(rot);
    }

    if let Some(trans) = translation {
      buf.translate(trans);
    }

    buf
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plane {
  #[serde(default)]
  pub position: Vector3,
  #[serde(default)]
  pub size: Vector3,
  #[serde(default)]
  pub rotation: Option<Vector3>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Triangle {
  pub points: [Vector3; 3],
  #[serde(default)]
  pub rotation: Option<Vector3>,
}

pub trait GenerateGeometry {
  fn generate_geometry(&self) -> GeometryBuffer;
}
