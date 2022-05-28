use std::str::Split;

use crate::Vertex;

#[derive(Debug)]
struct Face {
    vertices: [usize; 3],
    texture_coords: [usize; 3],
    normals: [usize; 3],
}

#[derive(Debug)]
pub struct Mesh {
    faces: Vec<Face>,
    vertices: Vec<[f32; 3]>,
    texture_coords: Vec<[f32; 2]>,
}

impl Mesh {
    pub fn from_string(data: &str) -> Mesh {
        let mut mesh = Mesh {
            faces: vec![],
            vertices: vec![],
            texture_coords: vec!(),
        };

        for line in data.lines() {
            let mut elements = line.split(" ");
            match elements.next() {
                Some(id) => match id {
                    "v" => {
                        mesh.vertices.push(load_vertex(elements));
                    }
                    "vt" => {
                        mesh.texture_coords.push(load_texture_coord(elements));
                    }
                    "f" => {
                        mesh.faces.push(load_face(elements));
                    }
                    _ => {}
                },
                None => {}
            }
        }

        mesh
    }

    pub(crate) fn vertices(&self) -> Vec<Vertex> {
        let mut vertices = vec![];
        for face in &self.faces {
            for i in face.vertices {
                vertices.push(Vertex {
                    position: self.vertices[i],
                    tex_coords: self.texture_coords[i],
                })
    
            }
        }

        vertices
    }
}

fn load_vertex(raw_vertices: Split<&str>) -> [f32; 3] {
    let mut vertex = [0.0; 3];

    for (i, elem) in raw_vertices.enumerate() {
        // TODO: Handle error here
        vertex[i] = elem.parse::<f32>().unwrap();
    }

    vertex
}

fn load_texture_coord(raw_coord: Split<&str>) -> [f32; 2] {
    let mut texture_coord = [0.0; 2];

    for (i, elem) in raw_coord.enumerate() {
        texture_coord[i] = elem.parse::<f32>().unwrap();
    }

    texture_coord
}

fn load_face(raw_face: Split<&str>) -> Face {
    let mut face = Face {
        vertices: [0; 3],
        texture_coords: [0; 3],
        normals: [0; 3],
    };

    for (i, elem) in raw_face.enumerate() {
        for (j, index) in elem.split("/").enumerate() {
            match j {
                // OBJ indexes from 1, subtract 1 for 0 based indexing
                0 => face.vertices[i] = index.parse::<usize>().unwrap() - 1,
                1 => face.texture_coords[i] = index.parse::<usize>().unwrap() - 1,
                2 => face.normals[i] = index.parse::<usize>().unwrap() - 1,
                _ => panic!("TODO: Fix this panic"),
            }
        }
    }

    face
}
