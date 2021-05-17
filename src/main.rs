use std::collections::HashMap;

pub use geometry::*;
pub use model::*;

pub mod gltf;
mod geometry;
mod model;

fn main() {
  for file in std::fs::read_dir("models").unwrap()
    .filter_map(|it| it.ok()) {
    let file_path_path = file.path();
    let file_path = file_path_path.to_str().unwrap();

    let file_name_os_str = file.file_name();
    let file_name = file_name_os_str.to_str().unwrap().split(".").nth(0).unwrap();
    let output_path = format!("output/{}", file_name);

    std::fs::create_dir_all("output").unwrap();

    if !(file_path.ends_with(".yml") || file_path.ends_with(".yaml")) {
      continue;
    }

    println!("Processing {}...", file_path);

    let src = std::fs::read_to_string(file.path()).unwrap();
    let model: Model = serde_yaml::from_str(&src).unwrap();

    let mut vertices = Vec::new();

    let mut accessors = Vec::new();
    let mut animations = Vec::new();
    let mut buffer_views = Vec::new();
    let mut materials = Vec::new();
    let mut meshes = Vec::new();
    let mut nodes = Vec::new();

    let mut material_indices = HashMap::new();
    let mut material_index_counter: u32 = 0;
    let mut mesh_indices = HashMap::new();
    let mut mesh_index_counter: u32 = 0;
    let mut node_ids = HashMap::new();
    let mut node_id_counter: u32 = 0;

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
        },
        double_sided: true,
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
          geometry.generate_vertices(&mut vertices);
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

        let byte_offset = start_vertices_len as u32 * 12;
        buffer_views.push(gltf::BufferView {
          buffer: 0,
          byte_offset,
          byte_length: (vertices.len() as u32 * 12) - byte_offset,
          target: Some(34962), // vertices
        });

        accessors.push(gltf::Accessor {
          buffer_view: buffer_views.len() as u32 - 1,
          byte_offset: 0,
          component_type: 5126,
          count: (vertices.len() - start_vertices_len) as u32,
          accessor_type: "VEC3".to_string(),
          max: vec![max.x, max.y, max.z],
          min: vec![min.x, min.y, min.z],
        });

        let material = primitive.material.as_ref()
          .map(|it| *material_indices.get(it).expect(&format!("Cannot find material '{}'", it)));

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

    let mut node_stack = Vec::new();
    let mut node_children = Vec::new();

    for node in &model.nodes {
      node_children.push(node);
    }

    while !node_children.is_empty() {
      let last = node_children.remove(node_children.len() - 1);
      for child in &last.children {
        node_children.push(child);
      }
      node_stack.push(last);
    }

    while !node_stack.is_empty() {
      let last = node_stack.remove(node_stack.len() - 1);
      node_ids.insert(last.name.clone(), node_id_counter);

      let mesh = last.mesh.as_ref().map(|it| *mesh_indices.get(it).unwrap());
      let children: Vec<u32> = last.children.iter()
        .map(|it| *node_ids.get(&it.name).unwrap())
        .collect();

      let rotation = last.rotation.map(|it| {
        let quat = glam::Quat::from_euler(glam::EulerRot::ZYX, it.x.to_radians(),
          it.y.to_radians(), it.z.to_radians())
          .normalize();
        [quat.x, quat.y, quat.z, quat.w]
      });

      nodes.push(gltf::Node {
        mesh,
        children,
        translation: last.offset.map(|it| [it.x, it.y, it.z]),
        rotation,
        scale: last.scale.map(|it| [it.x, it.y, it.z]),
      });

      node_id_counter += 1;
    }

    let mut animation_data = Vec::new();
    for animation in &model.animations {
      let mut gltf_animation = gltf::Animation::default();
      gltf_animation.name = animation.name.clone();

      for channel in &animation.channels {
        for node_name in &channel.nodes {
          let node = *node_ids.get(node_name).unwrap();

          let mut max_time = 0f32;

          let byte_offset = animation_data.len() as u32 * 4;
          for keyframe in &channel.keyframes {
            animation_data.push(keyframe.0);
            max_time = max_time.max(keyframe.0);
          }
          let output_byte_offset = animation_data.len() as u32 * 4 - byte_offset;

          let mut max_value = channel.keyframes[0].1;
          let mut min_value = channel.keyframes[0].1;

          let mut min_quat = glam::Vec4::new(1.0, 1.0, 1.0, 1.0);
          let mut max_quat = glam::Vec4::new(-1.0, -1.0, -1.0, -1.0);

          for keyframe in &channel.keyframes {
            if channel.target == Target::Rotation {
              let quat = glam::Quat::from_euler(
                glam::EulerRot::ZYX,
                keyframe.1.x.to_radians(),
                keyframe.1.y.to_radians(),
                keyframe.1.z.to_radians(),
              ).normalize();

              animation_data.push(quat.x);
              animation_data.push(quat.y);
              animation_data.push(quat.z);
              animation_data.push(quat.w);

              min_quat.x = min_quat.x.min(quat.x);
              min_quat.y = min_quat.y.min(quat.y);
              min_quat.z = min_quat.z.min(quat.z);
              min_quat.w = min_quat.w.min(quat.w);

              max_quat.x = max_quat.x.max(quat.x);
              max_quat.y = max_quat.y.max(quat.y);
              max_quat.z = max_quat.z.max(quat.z);
              max_quat.w = max_quat.w.max(quat.w);
            } else {
              animation_data.push(keyframe.1.x);
              animation_data.push(keyframe.1.y);
              animation_data.push(keyframe.1.z);

              min_value = min_value.min(keyframe.1);
              max_value = max_value.max(keyframe.1);
            }
          }

          let buffer_view = buffer_views.len() as u32;
          buffer_views.push(gltf::BufferView {
            buffer: 1,
            byte_offset,
            byte_length: animation_data.len() as u32 * 4 - byte_offset,
            target: None,
          });

          let input_sampler = gltf::Accessor {
            buffer_view,
            byte_offset: 0,
            component_type: 5126,
            count: channel.keyframes.len() as u32,
            accessor_type: "SCALAR".to_string(),
            max: vec![max_time],
            min: vec![0.0],
          };

          let is_quat = channel.target == Target::Rotation;

          let output_sampler = gltf::Accessor {
            buffer_view,
            byte_offset: output_byte_offset,
            component_type: 5126,
            count: channel.keyframes.len() as u32,
            accessor_type: if is_quat { "VEC4".to_string() } else { "VEC3".to_string() },
            max: if is_quat { max_quat.to_array().into() } else { max_value.into() },
            min: if is_quat { min_quat.to_array().into() } else { min_value.into() },
          };

          let input_sampler_id = accessors.len() as u32;
          accessors.push(input_sampler);

          let output_sampler_id = accessors.len() as u32;
          accessors.push(output_sampler);

          let path = match channel.target {
            Target::Translation => gltf::Path::Translation,
            Target::Rotation => gltf::Path::Rotation,
            Target::Scale => gltf::Path::Scale,
          };

          gltf_animation.channels.push(gltf::Channel {
            sampler: gltf_animation.samplers.len() as u32,
            target: gltf::Target { node, path },
          });
          gltf_animation.samplers.push(gltf::Sampler {
            input: input_sampler_id,
            output: output_sampler_id,
            interpolation: gltf::Interpolation::Linear,
          });
        }
      }

      animations.push(gltf_animation);
    }

    let base64_config = base64::Config::new(base64::CharacterSet::Standard, false);

    let mut output = gltf::Gltf {
      scene: 0,
      scenes: vec![gltf::Scene {
        nodes: model.nodes.iter().map(|it| *node_ids.get(&it.name).unwrap()).collect(),
      }],
      nodes,
      meshes,
      buffers: vec![gltf::Buffer {
        uri: format!("data:application/octet-stream;base64,{}",
          base64::encode_config(bytemuck::cast_slice(&vertices), base64_config)),
        byte_length: vertices.len() as u32 * 12,
      }],
      buffer_views,
      accessors,
      materials,
      animations,
      asset: gltf::Asset { version: "2.0".to_string() },
    };

    if animation_data.len() > 0 {
      output.buffers.push(gltf::Buffer {
        uri: format!("data:application/octet-stream;base64,{}",
          base64::encode_config(bytemuck::cast_slice(&animation_data), base64_config)),
        byte_length: animation_data.len() as u32 * 4,
      })
    }

    std::fs::write(&format!("{}.gltf", output_path),
      serde_json::to_string_pretty(&output).unwrap()).unwrap();
  }
}
