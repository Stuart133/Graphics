use std::str::Split;

use crate::Vertex;

#[derive(Debug)]
struct Face {
    vertices: Vec<usize>,
    texture_coords: Vec<usize>,
    normals: Vec<usize>,
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
            for i in &face.vertices {
                vertices.push(Vertex {
                    position: self.vertices[*i],
                    tex_coords: [0.0, 0.0],
                    // tex_coords: self.texture_coords[*i],
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
        if i >= 2 {
            continue;
        }
        texture_coord[i] = elem.parse::<f32>().unwrap();
    }

    texture_coord
}

fn load_face(raw_face: Split<&str>) -> Face {
    let mut face = Face {
        vertices: vec!(),
        texture_coords: vec!(),
        normals: vec!(),
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
