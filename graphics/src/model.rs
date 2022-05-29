use std::str::Split;

use wgpu::*;

pub trait Vertex {
    fn desc<'a>() -> VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    position: [f32; 3],
    texture_coords: [f32; 2],
    normal: [f32; 3],
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[derive(Debug)]
struct Face {
    vertices: Vec<usize>,
    texture_coords: Vec<usize>,
    normals: Vec<usize>,
}

#[derive(Debug)]
pub struct Model {
    faces: Vec<Face>,
    vertices: Vec<[f32; 3]>,
    texture_coords: Vec<[f32; 2]>,
}

impl Model {}

fn load_face(raw_face: Split<&str>) -> Face {
    let mut face = Face {
        vertices: vec![],
        texture_coords: vec![],
        normals: vec![],
    };

    for elem in raw_face {
        if elem == "" {
            continue;
        }
        for (j, raw_index) in elem.split("/").enumerate() {
            let index = raw_index.parse::<usize>().unwrap();
            if index == 0 {
                continue;
            }

            match j {
                // OBJ indexes from 1, subtract 1 for 0 based indexing
                0 => face.vertices.push(index - 1),
                1 => face.texture_coords.push(index - 1),
                2 => face.normals.push(index - 1),
                _ => panic!("TODO: Fix this panic"),
            }
        }
    }

    face
}
