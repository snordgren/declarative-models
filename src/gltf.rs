use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
  pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gltf {
  pub scene: u32,
  pub scenes: Vec<Scene>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub nodes: Vec<Node>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub meshes: Vec<Mesh>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub buffers: Vec<Buffer>,
  #[serde(rename = "bufferViews")]
  pub buffer_views: Vec<BufferView>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub accessors: Vec<Accessor>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub materials: Vec<Material>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub animations: Vec<Animation>,
  pub asset: Asset,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scene {
  pub nodes: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub mesh: Option<u32>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub children: Vec<u32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub translation: Option<[f32; 3]>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub rotation: Option<[f32; 4]>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub scale: Option<[f32; 3]>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mesh {
  pub primitives: Vec<Primitive>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Primitive {
  pub attributes: Attributes,
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub material: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attributes {
  #[serde(rename = "POSITION")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub position: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Buffer {
  pub uri: String,
  #[serde(rename="byteLength")]
  pub byte_length: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BufferView {
  pub buffer: u32,
  #[serde(rename = "byteOffset")]
  pub byte_offset: u32,
  #[serde(rename = "byteLength")]
  pub byte_length: u32,
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub target: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Accessor {
  #[serde(rename="bufferView")]
  pub buffer_view: u32,
  #[serde(rename="byteOffset")]
  pub byte_offset: u32,
  #[serde(rename="componentType")]
  pub component_type: u32,
  pub count: u32,
  #[serde(rename="type")]
  pub accessor_type: String,
  pub max: Vec<f32>,
  pub min: Vec<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Material {
  pub name: String,
  #[serde(rename = "pbrMetallicRoughness")]
  pub pbr_metallic_roughness: PBRMetallicRoughness,
  #[serde(rename = "doubleSided")]
  pub double_sided: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PBRMetallicRoughness {
  #[serde(rename = "baseColorFactor")]
  pub base_color_factor: [f32; 4],
  #[serde(rename = "metallicFactor")]
  pub metallic_factor: f32,
  #[serde(rename = "roughnessFactor")]
  pub roughness_factor: f32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Animation {
  pub name: String,
  pub samplers: Vec<Sampler>,
  pub channels: Vec<Channel>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sampler {
  pub input: u32,
  pub output: u32,
  pub interpolation: Interpolation,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
  pub sampler: u32,
  pub target: Target,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Target {
  pub node: u32,
  pub path: Path,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Interpolation {
  #[serde(rename = "LINEAR")]
  Linear,
  #[serde(rename = "STEP")]
  Step,
  #[serde(rename = "CUBICSPLINE")]
  CubicSpline,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Path {
  #[serde(rename = "rotation")]
  Rotation,
  #[serde(rename = "scale")]
  Scale,
  #[serde(rename = "translation")]
  Translation,
}
