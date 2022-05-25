use std::str::Split;

use crate::Vertex;

#[derive(Debug)]
pub struct Mesh {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u16>,
}

impl Mesh {
    pub fn from_string(data: &str) -> Mesh {
        let mut mesh = Mesh {
            vertices: vec![],
            indices: vec![],
        };

        for line in data.lines() {
            let mut elements = line.split(" ");
            match elements.next() {
                Some(id) => match id {
                    "v" => {
                        mesh.vertices.push(load_vertex(elements));
                    },
                    "f" => {
                      load_indices(elements, &mut mesh.indices);
                    }
                    _ => {}
                },
                None => {}
            }
        }

        mesh
    }
}

fn load_vertex(raw_vertices: Split<&str>) -> Vertex {
    let mut vertex = Vertex {
        position: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    };

    for (i, elem) in raw_vertices.enumerate() {
        // TODO: Handle error here
        vertex.position[i] = elem.parse::<f32>().unwrap();
    }

    vertex
}

fn load_indices<'a>(raw_face: Split<&str>, indices: &mut Vec<u16>) {
  for elem in raw_face {
    match elem.split("/").next() {
        Some(index) => {
          // TODO: Handle error here
          indices.push(index.parse::<u16>().unwrap() - 1) // OBJ indexes from 1, subtract 1 for 0 based indexing
        },
        None => todo!(),  // TODO: Handle error here
    }
  }
}
