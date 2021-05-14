use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
  pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gltf {
  pub scene: u32,
  pub scenes: Vec<Scene>,
  pub nodes: Vec<Node>,
  pub meshes: Vec<Mesh>,
  pub buffers: Vec<Buffer>,
  #[serde(rename="bufferViews")]
  pub buffer_views: Vec<BufferView>,
  pub accessors: Vec<Accessor>,
  pub asset: Asset,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scene {
  pub nodes: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
  pub mesh: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mesh {
  pub primitives: Vec<Primitive>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Primitive {
  pub attributes: Attributes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attributes {
  #[serde(rename = "POSITION")]
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
  #[serde(rename="byteOffset")]
  pub byte_offset: u32,
  #[serde(rename="byteLength")]
  pub byte_length: u32,
  pub target: u32,
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
