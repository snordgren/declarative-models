use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod gltf;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Material {
  pub name: String,
  #[serde(rename = "baseColor")]
  pub base_color: [f32; 4],
  pub metallic: f32,
  pub roughness: f32,
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

    let mut materials = Vec::new();
    let mut meshes = Vec::new();
    let mut buffer_views = Vec::new();
    let mut accessors = Vec::new();

    let mut material_indices = HashMap::new();
    let mut material_index_counter = 0u32;
    let mut mesh_indices = HashMap::new();
    let mut mesh_index_counter = 0;

    for material in &model.materials {
      let mut base_color = material.base_color.clone();

      let is_large = base_color.iter().any(|it| *it > 1.0);
      if is_large {
        for i in 0..base_color.len() {
          base_color[i] /= 255.0;
        }
      }

      materials.push(gltf::Material {
        name: material.name.clone(),
        pbr_metallic_roughness: gltf::PBRMetallicRoughness {
          base_color_factor: base_color,
          metallic_factor: material.metallic,
          roughness_factor: material.roughness,
        }
      });

      material_indices.insert(material.name.clone(), material_index_counter);
      material_index_counter += 1;
    }

    for mesh in &model.meshes {
      let mut primitives = Vec::new();

      for primitive in &mesh.primitives {
        let start_vertices_len = vertices.len();
        let mut min = Vector3 { x: 256.0, y: 256.0, z: 256.0 };
        let mut max = Vector3::default();

        for geometry in &primitive.geometry {
          match geometry {
            Geometry::Box(_) => {}
            Geometry::Triangle(triangle) => {
              vertices.push(triangle.points[0]);
              vertices.push(triangle.points[1]);
              vertices.push(triangle.points[2]);
            }
          }
        }

        for i in start_vertices_len..vertices.len() {
          let vertex = vertices[i];
          if vertex.x < min.x {
            min.x = vertex.x;
          }
          if vertex.y < min.y {
            min.y = vertex.y;
          }
          if vertex.z < min.z {
            min.z = vertex.z;
          }
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

        buffer_views.push(gltf::BufferView {
          buffer: 0,
          byte_offset: start_vertices_len as u32 * 12,
          byte_length: vertices.len() as u32 * 12,
          target: 34962, // vertices
        });

        accessors.push(gltf::Accessor {
          buffer_view: buffer_views.len() as u32 - 1,
          byte_offset: 0,
          component_type: 5126,
          count: vertices.len() as u32,
          accessor_type: "VEC3".to_string(),
          max: vec![max.x, max.y, max.z],
          min: vec![min.x, min.y, min.z],
        });

        let material = primitive.material.as_ref()
          .map(|it| *material_indices.get(it).unwrap());

        primitives.push(gltf::Primitive {
          attributes: gltf::Attributes {
            position: Some(accessors.len() as u32 - 1),
          },
          material,
        });
      }

      meshes.push(gltf::Mesh {
        primitives
      });

      mesh_indices.insert(mesh.name.clone(), mesh_index_counter);
      mesh_index_counter += 1;
    }

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

    let mut nodes = Vec::new();
    for node in &model.nodes {
      let mesh = node.mesh.as_ref().map(|it| *mesh_indices.get(it).unwrap());

      nodes.push(gltf::Node {
        mesh,
      });
    }

    let output = gltf::Gltf {
      scene: 0,
      scenes: vec![gltf::Scene {
        nodes: vec![0]
      }],
      nodes,
      meshes,
      buffers: vec![gltf::Buffer {
        uri: format!("{}.bin", file_name),
        byte_length: vertices.len() as u32 * 12,
      }],
      buffer_views,
      accessors,
      materials,
      asset: gltf::Asset { version: "2.0".to_string() },
    };

    std::fs::write(&format!("{}/{}.gltf", output_path, file_name),
      serde_json::to_string_pretty(&output).unwrap()).unwrap();

    std::fs::write(format!("{}/{}.bin", output_path, file_name),
      bytemuck::cast_slice(&vertices)).unwrap();
  }
}
