use rand::prelude::*;
use rand_pcg::Pcg64;

use crate::Vector3;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Triangle(pub [u16; 3]);

#[derive(Clone, Debug, Default)]
pub struct GeometryBuffer {
  pub vertices: Vec<Vector3>,
  pub triangles: Vec<Triangle>,
}

impl GeometryBuffer {
  pub fn apply_transform(&mut self, matrix: glam::Mat4) {
    for i in 0..(self.vertices.len()) {
      let v0 = self.vertices[i];
      let v1 = glam::vec3(v0.x, v0.y, v0.z);
      let v2 = matrix.transform_point3(v1);
      self.vertices[i].set(v2.x, v2.y, v2.z);
    }
  }

  pub fn deform(&mut self, min: Vector3, max: Vector3, scale: Vector3, seed: u64) {
    let mut rng = Pcg64::seed_from_u64(seed);

    for i in 0..self.vertices.len() {
      let min_x = min.x * scale.x;
      let x_range = (max.x * scale.y) - min_x;
      let x_offset = min_x + rng.gen::<f32>() * x_range;

      let min_y = min.y * scale.y;
      let y_range = (max.y * scale.y) - min_y;
      let y_offset = min_y + rng.gen::<f32>() * y_range;

      let min_z = min.z * scale.z;
      let z_range = (max.z * scale.z) - min_z;
      let z_offset = min_z + rng.gen::<f32>() * z_range;

      self.vertices[i] += Vector3::new(x_offset, y_offset, z_offset);
    }
  }

  /// Transform the geometry buffer into a non-indexed array of vertices.
  pub fn make_redundant(&self) -> Vec<Vector3> {
    let mut output = Vec::new();
    for i in 0..self.triangles.len() {
      let triangle = self.triangles[i];

      let f0 = triangle.0[0] as usize;
      let f1 = triangle.0[1] as usize;
      let f2 = triangle.0[2] as usize;

      output.push(self.vertices[f0]);
      output.push(self.vertices[f1]);
      output.push(self.vertices[f2]);
    }
    output
  }

  pub fn max_position(&self) -> Vector3 {
    let mut max = Vector3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);
    for vertex in &self.vertices {
      max.x = max.x.max(vertex.x);
      max.y = max.y.max(vertex.y);
      max.z = max.z.max(vertex.z);
    }
    max
  }

  pub fn min_position(&self) -> Vector3 {
    let mut min = Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
    for vertex in &self.vertices {
      min.x = min.x.min(vertex.x);
      min.y = min.y.min(vertex.y);
      min.z = min.z.min(vertex.z);
    }
    min
  }

  pub fn new() -> Self {
    Self::default()
  }

  pub fn remove_triangle(&mut self, index: usize) {
    self.triangles.remove(index);
  }

  pub fn rotate(&mut self, rot: Vector3) {
    let matrix = glam::Mat4::from_euler(glam::EulerRot::ZYX,
      rot.x.to_radians(), rot.y.to_radians(), rot.z.to_radians());

    self.apply_transform(matrix);
  }

  pub fn scale(&mut self, scale: Vector3) {
    let matrix = glam::Mat4::from_scale(
      glam::vec3(scale.x, scale.y, scale.z),
    );

    self.apply_transform(matrix);
  }

  pub fn split_triangle(&mut self, index: usize, p_index: u16) {
    let triangle = self.triangles[index];

    self.triangles.remove(index);

    self.triangles.push(Triangle([triangle.0[0], triangle.0[1], p_index]));
    self.triangles.push(Triangle([triangle.0[1], triangle.0[2], p_index]));
    self.triangles.push(Triangle([triangle.0[2], p_index, triangle.0[0]]));
  }

  pub fn translate(&mut self, trans: Vector3) {
    let matrix = glam::Mat4::from_translation(
      glam::vec3(trans.x, trans.y, trans.z));

    self.apply_transform(matrix);
  }

  pub fn triangle(&mut self, a: u16, b: u16, c: u16) {
    if a == b || b == c || c == a {
      panic!(format!("Cannot add triangle with duplicate indices {}, {}, {}", a, b, c));
    }

    self.triangles.push(Triangle([a, b, c]));
  }

  pub fn vertex(&mut self, pos: Vector3) -> u16 {
    let index = self.vertices.len();
    self.vertices.push(pos);

    let max_index = u16::max_value() as usize;
    if index >= max_index {
      panic!(format!("Too many vertices n={} (max is {}).", index, max_index));
    }

    index as u16
  }
}
