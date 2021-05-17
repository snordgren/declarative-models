use rand::prelude::*;
use rand_pcg::Pcg64;

use serde::{Deserialize, Serialize};

use crate::Vector3;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Geometry {
  Cube(Cube),
  Cylinder(Cylinder),
  Plane(Plane),
  Triangle(Triangle),
}

impl Geometry {
  pub fn generate_vertices(&self, vertices: &mut Vec<Vector3>) {
    let start = vertices.len();
    let rotation;
    let translation;

    match self {
      Geometry::Cube(b) => {
        let min = b.position - b.size / Vector3::new(2.0, 2.0, 2.0);
        let max = min + b.size;

        let offset = b.offsets.unwrap_or_default();

        let p000 = min + offset.v000;
        let p001 = Vector3::new(min.x, min.y, max.z) + offset.v001;
        let p010 = Vector3::new(min.x, max.y, min.z) + offset.v010;
        let p011 = Vector3::new(min.x, max.y, max.z) + offset.v011;
        let p100 = Vector3::new(max.x, min.y, min.z) + offset.v100;
        let p101 = Vector3::new(max.x, min.y, max.z) + offset.v101;
        let p110 = Vector3::new(max.x, max.y, min.z) + offset.v110;
        let p111 = Vector3::new(max.x, max.y, max.z) + offset.v111;

        vertices.push(p000);
        vertices.push(p100);
        vertices.push(p010);
        vertices.push(p010);
        vertices.push(p100);
        vertices.push(p110);

        vertices.push(p001);
        vertices.push(p101);
        vertices.push(p011);
        vertices.push(p011);
        vertices.push(p101);
        vertices.push(p111);

        vertices.push(p000);
        vertices.push(p001);
        vertices.push(p100);
        vertices.push(p100);
        vertices.push(p001);
        vertices.push(p101);

        vertices.push(p010);
        vertices.push(p011);
        vertices.push(p110);
        vertices.push(p110);
        vertices.push(p011);
        vertices.push(p111);

        vertices.push(p000);
        vertices.push(p010);
        vertices.push(p001);
        vertices.push(p001);
        vertices.push(p010);
        vertices.push(p011);

        vertices.push(p100);
        vertices.push(p110);
        vertices.push(p101);
        vertices.push(p101);
        vertices.push(p110);
        vertices.push(p111);

        rotation = b.rotation;
        translation = None;
      }
      Geometry::Cylinder(c) => {
        fn calculate_angle(points: u32, i: u32) -> f32 {
          let mut i0 = i;
          if i0 >= points {
            i0 -= points;
          }
          (i0 as f32) * (2.0 * std::f32::consts::PI) / (points as f32)
        }

        let z0 = -0.5 * c.size.z;
        let z1 = 0.5 * c.size.z;

        let center_z0 = Vector3::new(c.size.x / 4.0, c.size.y / 4.0, z0);
        let center_z1 = Vector3::new(c.size.x / 4.0, c.size.y / 4.0, z1);

        let mut vertex_points: Vec<[f32; 2]> = (0..c.points)
          .map(|i| {
            let radians = calculate_angle(c.points, i);
            let x = radians.cos() * 0.5 * c.size.x;
            let y = radians.sin() * 0.5 * c.size.y;
            [x, y]
          })
          .collect();

        if let Some(displacement) = &c.displacement {
          let mut rng = Pcg64::seed_from_u64(displacement.seed);
          for i in 0..vertex_points.len() {
            let min_x = displacement.min.x;
            let x_range = displacement.max.x - min_x;
            let x_offset = min_x + rng.gen::<f32>() * x_range;

            let min_y = displacement.min.y;
            let y_range = displacement.max.y - min_y;
            let y_offset = min_y + rng.gen::<f32>() * y_range;

            vertex_points[i][0] += x_offset;
            vertex_points[i][1] += y_offset;
          }
        }

        for i in 0..c.points {
          let [x0, y0] = vertex_points[i as usize];
          let [x1, y1] = vertex_points[(i as usize + 1) % vertex_points.len()];

          let v000 = Vector3::new(x0, y0, z0);
          let v001 = Vector3::new(x0, y0, z1);
          let v110 = Vector3::new(x1, y1, z0);
          let v111 = Vector3::new(x1, y1, z1);

          vertices.push(v000);
          vertices.push(v110);
          vertices.push(center_z0);

          vertices.push(v001);
          vertices.push(v111);
          vertices.push(center_z1);

          vertices.push(v000);
          vertices.push(v110);
          vertices.push(v111);

          vertices.push(v000);
          vertices.push(v001);
          vertices.push(v111);
        }

        rotation = c.rotation;
        translation = Some(c.position);
      }
      Geometry::Plane(p) => {
        let min = p.position - p.size / Vector3::new(2.0, 2.0, 2.0);
        let max = min + p.size;

        let v00 = Vector3::new(min.x, 0.0, min.y);
        let v01 = Vector3::new(min.x, 0.0, max.y);
        let v10 = Vector3::new(max.x, 0.0, min.y);
        let v11 = Vector3::new(max.x, 0.0, max.y);

        vertices.push(v00);
        vertices.push(v10);
        vertices.push(v01);
        vertices.push(v01);
        vertices.push(v10);
        vertices.push(v11);

        rotation = p.rotation;
        translation = None;
      }
      Geometry::Triangle(triangle) => {
        vertices.push(triangle.points[0]);
        vertices.push(triangle.points[1]);
        vertices.push(triangle.points[2]);

        rotation = triangle.rotation;
        translation = None;
      }
    }

    if let Some(rot) = rotation {
      let rotation_matrix = glam::Mat4::from_euler(glam::EulerRot::ZYX,
        rot.x.to_radians(), rot.y.to_radians(), rot.z.to_radians());

      for i in start..(vertices.len()) {
        let v0 = vertices[i];
        let v1 = glam::vec3(v0.x, v0.y, v0.z);
        let v2 = rotation_matrix.transform_point3(v1);
        vertices[i].set(v2.x, v2.y, v2.z);
      }
    }

    if let Some(trans) = translation {
      let matrix = glam::Mat4::from_translation(
        glam::vec3(trans.x, trans.y, trans.z));

      for i in start..(vertices.len()) {
        let v0 = vertices[i];
        let v1 = glam::vec3(v0.x, v0.y, v0.z);
        let v2 = matrix.transform_point3(v1);
        vertices[i].set(v2.x, v2.y, v2.z);
      }
    }
  }
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cylinder {
  #[serde(default)]
  pub position: Vector3,
  #[serde(default)]
  pub size: Vector3,
  pub points: u32,
  #[serde(default)]
  pub rotation: Option<Vector3>,
  #[serde(default)]
  pub displacement: Option<Displacement>,
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
