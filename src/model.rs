use std::ops::{Add, Div, Mul, Sub};

use serde::{Deserialize, Serialize};

use crate::Geometry;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Model {
  pub meshes: Vec<Mesh>,
  pub nodes: Vec<Node>,
  #[serde(default)]
  pub animations: Vec<Animation>,
  #[serde(default)]
  pub materials: Vec<Material>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mesh {
  pub name: String,
  pub primitives: Vec<Primitive>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Primitive {
  #[serde(default)]
  pub material: Option<String>,
  pub geometry: Vec<Geometry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
  pub name: String,
  #[serde(default)]
  pub mesh: Option<String>,
  #[serde(default)]
  pub offset: Option<Vector3>,
  #[serde(default)]
  pub rotation: Option<Vector3>,
  #[serde(default)]
  pub scale: Option<Vector3>,
  #[serde(default)]
  pub children: Vec<Node>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Animation {
  pub name: String,
  pub channels: Vec<Channel>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
  pub nodes: Vec<String>,
  pub target: Target,
  pub keyframes: Vec<(f32, Vector3)>,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[repr(C)]
pub struct Vector3 {
  #[serde(default)]
  pub x: f32,
  #[serde(default)]
  pub y: f32,
  #[serde(default)]
  pub z: f32,
}

unsafe impl bytemuck::Zeroable for Vector3 {}

unsafe impl bytemuck::Pod for Vector3 {}

impl Vector3 {
  pub const ZERO: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
  pub const ONE: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
  pub const MINUS_ONE: Vector3 = Vector3 { x: -1.0, y: -1.0, z: -1.0 };

  pub fn max(&self, other: Vector3) -> Self {
    Self {
      x: self.x.max(other.x),
      y: self.y.max(other.y),
      z: self.z.max(other.z),
    }
  }

  pub fn min(&self, other: Vector3) -> Self {
    Self {
      x: self.x.min(other.x),
      y: self.y.min(other.y),
      z: self.z.min(other.z),
    }
  }

  pub fn minus_one() -> Self {
    Vector3::MINUS_ONE
  }

  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  pub fn one() -> Self {
    Vector3::ONE
  }

  pub fn set(&mut self, x: f32, y: f32, z: f32) {
    self.x = x;
    self.y = y;
    self.z = z;
  }
}

impl Add for Vector3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Div for Vector3 {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
      z: self.z / rhs.z,
    }
  }
}

impl Mul for Vector3 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z,
    }
  }
}

impl Sub for Vector3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl From<Vector3> for [f32; 3] {
  fn from(xyz: Vector3) -> Self {
    [xyz.x, xyz.y, xyz.z]
  }
}

impl From<Vector3> for Vec<f32> {
  fn from(xyz: Vector3) -> Self {
    vec![xyz.x, xyz.y, xyz.z]
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Material {
  pub name: String,
  #[serde(rename = "baseColor")]
  pub base_color: [f32; 4],
  #[serde(default)]
  pub metallic: f32,
  #[serde(default)]
  pub roughness: f32,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Target {
  Translation,
  Rotation,
  Scale,
}
