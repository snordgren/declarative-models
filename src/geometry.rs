use rand::prelude::*;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

pub use cone::*;
pub use cube::*;
pub use deform::*;
pub use icosphere::*;
pub use uv_sphere::*;

use crate::{GeometryBuffer, Vector3};

mod cone;
mod cube;
mod deform;
mod icosphere;
mod uv_sphere;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Geometry {
  Cone(Cone),
  Cube(Cube),
  Cylinder(Cylinder),
  Deform(Box<Deform>),
  IcoSphere(Icosphere),
  Plane(Plane),
  Triangle(Triangle),
  UvSphere(UvSphere),
}

impl Geometry {
  pub fn generate_vertices(&self) -> GeometryBuffer {
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
        fn calculate_angle(points: u32, i: u32) -> f32 {
          let mut i0 = i;
          if i0 >= points {
            i0 -= points;
          }
          (i0 as f32) * (2.0 * std::f32::consts::PI) / (points as f32)
        }

        let z0 = -0.5;
        let z1 = 0.5;

        let center_x = 0.0;
        let center_y = 0.0;
        let center_z0 = buf.vertex(Vector3::new(center_x, center_y, z0));
        let center_z1 = buf.vertex(Vector3::new(center_x, center_y, z1));

        let mut vertex_points: Vec<[f32; 2]> = (0..c.points)
          .map(|i| {
            let radians = calculate_angle(c.points, i);
            let x = radians.cos() * 0.5;
            let y = radians.sin() * 0.5;
            [x, y]
          })
          .collect();

        let mut indices = Vec::new();
        for i in 0..c.points {
          let [x0, y0] = vertex_points[i as usize];

          let v000 = buf.vertex(Vector3::new(x0, y0, z0));
          let v001 = buf.vertex(Vector3::new(x0, y0, z1));
          indices.push(v000);
          indices.push(v001);
        }

        for i in 0..c.points {
          let [x0, y0] = vertex_points[i as usize];
          let [x1, y1] = vertex_points[(i as usize + 1) % vertex_points.len()];

          let v000 = indices[2 * i as usize];
          let v001 = indices[2 * i as usize + 1];
          let v110 = indices[2 * ((i as usize + 1) % vertex_points.len())];
          let v111 = indices[2 * ((i as usize + 1) % vertex_points.len()) + 1];

          buf.triangle(v000, v110, center_z0);
          buf.triangle(v001, v111, center_z1);
          buf.triangle(v000, v110, v111);
          buf.triangle(v000, v001, v111);
        }

        buf.rotate(Vector3::new(0.0, 0.0, 90.0));

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
      Geometry::IcoSphere(i) => {
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
pub struct Cylinder {
  #[serde(default)]
  pub position: Vector3,
  #[serde(default)]
  pub size: Vector3,
  pub points: u32,
  #[serde(default)]
  pub rotation: Option<Vector3>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Displacement {
  #[serde(default)]
  pub seed: u64,
  #[serde(default = "Vector3::minus_one")]
  pub min: Vector3,
  #[serde(default = "Vector3::one")]
  pub max: Vector3,
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
