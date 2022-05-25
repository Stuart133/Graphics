use std::str::Split;

use crate::Vertex;

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
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
                    }
                    _ => {}
                },
                None => {}
            }
        }

        mesh
    }
}

fn load_vertex(split: Split<&str>) -> Vertex {
    let mut vertex = Vertex {
        position: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    };

    for (i, elem) in split.enumerate() {
        // TODO: Handle error here
        vertex.position[i] = elem.parse::<f32>().unwrap();
    }

    vertex
}
