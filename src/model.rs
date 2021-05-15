use serde::{Deserialize, Serialize};

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
pub enum Geometry {
  Box(Box),
  Triangle(Triangle),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Box {
  #[serde(default)]
  pub position: Vector3,
  #[serde(default)]
  pub size: Vector3,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Triangle {
  pub points: [Vector3; 3],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
  pub name: String,
  #[serde(default)]
  pub mesh: Option<String>,
  #[serde(default)]
  pub offset: Vector3,
  #[serde(default)]
  pub rotation: Vector3,
  #[serde(default)]
  pub children: Vec<Node>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Animation {
  pub name: String,
  pub node: String,
  pub target: Target,
  pub keyframes: Vec<(f32, Vector3)>,
}
/*
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Keyframe {
  pub time: f32,
  #[serde(default)]
  pub rotation: Option<Vector3>,
  #[serde(default)]
  pub translation: Option<Vector3>,
}*/

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
  pub fn max(&self, other: Vector3) -> Vector3 {
    Vector3 {
      x: self.x.max(other.x),
      y: self.y.max(other.y),
      z: self.z.max(other.z),
    }
  }

  pub fn min(&self, other: Vector3) -> Vector3 {
    Vector3 {
      x: self.x.min(other.x),
      y: self.y.min(other.y),
      z: self.z.min(other.z),
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
  pub metallic: f32,
  pub roughness: f32,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Target {
  Translation,
  Rotation,
  Scale,
}
