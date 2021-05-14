use serde::{Deserialize, Serialize};

pub mod gltf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Model {
  pub meshes: Vec<Mesh>,
  pub nodes: Vec<Node>,
  #[serde(default)]
  pub animations: Vec<Animation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mesh {
  pub name: String,
  pub geometry: Vec<Geometry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Geometry {
  Box(Box),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Box {
  #[serde(default)]
  pub position: Vector3,
  #[serde(default)]
  pub size: Vector3,
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

fn main() {
  for file in std::fs::read_dir("models").unwrap()
    .filter_map(|it| it.ok()) {

    let file_path_path = file.path();
    let file_path = file_path_path.to_str().unwrap();

    let file_name_os_str = file.file_name();
    let file_name = file_name_os_str.to_str().unwrap().split(".").nth(0).unwrap();
    let output_path = format!("output/{}", file_name);

    std::fs::create_dir_all(&output_path).unwrap();

    if !(file_path.ends_with(".yml") || file_path.ends_with(".yaml")) {
      continue;
    }

    let src = std::fs::read_to_string(file.path()).unwrap();
    let model: Model = serde_yaml::from_str(&src).unwrap();

    let mut vertices = Vec::new();
    vertices.push(Vector3 { x: 0.0, y: 0.0, z: 0.0 });
    vertices.push(Vector3 { x: 1.0, y: 0.0, z: 0.0 });
    vertices.push(Vector3 { x: 0.5, y: 1.0, z: 0.0 });

    let mut max = Vector3::default();
    for vertex in &vertices {
      if vertex.x > max.x {
        max.x = vertex.x;
      }
      if vertex.y > max.y {
        max.y = vertex.y;
      }
      if vertex.z > max.z {
        max.z = vertex.z;
      }
    }

    let output = gltf::Gltf {
      scene: 0,
      scenes: vec![gltf::Scene {
        nodes: vec![0]
      }],
      nodes: vec![gltf::Node {
        mesh: Some(0),
      }],
      meshes: vec![gltf::Mesh {
        primitives: vec![gltf::Primitive {
          attributes: gltf::Attributes {
            position: Some(0),
          },
        }]
      }],
      buffers: vec![gltf::Buffer {
        uri: format!("{}.bin", file_name),
        byte_length: vertices.len() as u32 * 12,
      }],
      buffer_views: vec![
        gltf::BufferView {
          buffer: 0,
          byte_offset: 0,
          byte_length: vertices.len() as u32 * 12,
          target: 34962, // vertices
        }],
      accessors: vec![
        gltf::Accessor {
          buffer_view: 0,
          byte_offset: 0,
          component_type: 5126,
          count: vertices.len() as u32,
          accessor_type: "VEC3".to_string(),
          max: vec![ max.x, max.y, max.z ],
          min: vec![ 0.0, 0.0, 0.0 ]
        }
      ],
      asset: gltf::Asset { version: "2.0".to_string() },
    };

    std::fs::write(&format!("{}/{}.gltf", output_path, file_name),
      serde_json::to_string_pretty(&output).unwrap()).unwrap();

    std::fs::write(format!("{}/{}.bin", output_path, file_name),
      bytemuck::cast_slice(&vertices)).unwrap();
  }
}
