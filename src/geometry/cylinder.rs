use serde::{Deserialize, Serialize};

use crate::{GenerateGeometry, GeometryBuffer, Vector3};

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

impl GenerateGeometry for Cylinder {
  fn generate_geometry(&self) -> GeometryBuffer {
    let mut buf = GeometryBuffer::new();

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

    let mut indices = Vec::new();
    for i in 0..self.points {
      let radians = calculate_angle(self.points, i);
      let x0 = radians.cos() * 0.5;
      let y0 = radians.sin() * 0.5;

      let v000 = buf.vertex(Vector3::new(x0, y0, z0));
      let v001 = buf.vertex(Vector3::new(x0, y0, z1));
      indices.push(v000);
      indices.push(v001);
    }

    for i in 0..self.points {
      let v000 = indices[2 * i as usize];
      let v001 = indices[2 * i as usize + 1];
      let v110 = indices[2 * ((i as usize + 1) % self.points as usize)];
      let v111 = indices[2 * ((i as usize + 1) % self.points as usize) + 1];

      buf.triangle(v000, v110, center_z0);
      buf.triangle(v001, v111, center_z1);

      buf.triangle(v000, v110, v111);
      buf.triangle(v000, v001, v111);
    }

    buf.rotate(Vector3::new(0.0, 0.0, 90.0));
    buf
  }
}
