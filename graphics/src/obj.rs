//! A simple .obj mesh loader

use std::{collections::HashMap, str::Split};

use crate::model::ModelVertex;

#[derive(Debug)]
pub struct Mesh {
    /// Vector of mesh vertices
    pub vertices: Vec<ModelVertex>,

    /// Vector of vertex indices
    pub indices: Vec<u32>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: Default::default(),
            indices: Default::default(),
        }
    }
}

impl Mesh {
    pub fn from_str(raw_mesh: &str) -> Result<Mesh, ObjLoadError> {
        let mut loader = MeshLoader::default();

        for line in raw_mesh.lines() {
            let mut elements = line.split(" ");
            match elements.next() {
                Some(key) => match key {
                    "v" => match loader.load_vertex(elements) {
                        Ok(_) => {}
                        Err(err) => return Err(err),
                    },
                    "vt" => match loader.load_texture_coord(elements) {
                        Ok(_) => {}
                        Err(err) => return Err(err),
                    },
                    "vn" => match loader.load_normal(elements) {
                        Ok(_) => {}
                        Err(err) => return Err(err),
                    },
                    "f" => match loader.load_face(elements) {
                        Ok(_) => {}
                        Err(err) => return Err(err),
                    },
                    _ => {} // Just ignore any unrecognised key
                },
                None => {}
            }
        }

        Ok(loader.export_faces())
    }
}

struct MeshLoader {
    faces: Vec<Face>,
    positions: Vec<[f32; 3]>,
    texture_coords: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
}

impl Default for MeshLoader {
    fn default() -> Self {
        Self {
            faces: Default::default(),
            positions: Default::default(),
            texture_coords: Default::default(),
            normals: Default::default(),
        }
    }
}

impl MeshLoader {
    fn export_faces(&self) -> Mesh {
        let mut mesh = Mesh::default();
        let mut vertex_map = Default::default();

        for face in self.faces.iter() {
            self.export_face(face, &mut mesh, &mut vertex_map);
        }

        mesh
    }

    fn export_face(
        &self,
        face: &Face,
        mesh: &mut Mesh,
        vertex_map: &mut HashMap<VertexIndices, usize>,
    ) {
        match face {
            // Ignore points
            Face::Point(_) => {}
            // Ignore lines
            Face::Line(_) => {}
            Face::Triangle(vertex_indices) => {
                for vi in vertex_indices {
                    self.export_vertex(vi, mesh, vertex_map);
                }
            }
            Face::Quad(_) => println!("{:?}", face),
        }
    }

    fn export_vertex(
        &self,
        indices: &VertexIndices,
        mesh: &mut Mesh,
        vertex_map: &mut HashMap<VertexIndices, usize>,
    ) {
        let index = vertex_map.get(&indices);
        match index {
            Some(index) => mesh.indices.push(*index as u32),
            None => {
                let vertex = ModelVertex::new(
                    self.positions[indices.position],
                    self.texture_coords[indices.texture_coord],
                    self.normals[indices.normal],
                );

                let index = mesh.vertices.len();
                mesh.indices.push(index as u32);
                mesh.vertices.push(vertex);

                vertex_map.insert(*indices, index);
            }
        }
    }

    fn load_face(&mut self, raw_face: Split<&str>) -> Result<(), ObjLoadError> {
        let mut face = vec![];

        for group in raw_face {
            let mut indices = VertexIndices::default();
            for (i, index) in group.split("/").enumerate() {
                // Value could be missing - If it is skip to next value
                if index == "" {
                    continue;
                }

                match index.parse::<usize>() {
                    Ok(index) => match i {
                        // OBJ indices are 1 based, adjust to 0 based
                        0 => indices.position = index - 1,
                        1 => indices.texture_coord = index - 1,
                        2 => indices.normal = index - 1,
                        _ => return Err(ObjLoadError::InvalidFaceValue),
                    },
                    Err(_) => return Err(ObjLoadError::InvalidFaceValue),
                }
            }

            face.push(indices);
        }

        match face.len() {
            1 => self.faces.push(Face::Point([face[0]])),
            2 => self.faces.push(Face::Line([face[0], face[1]])),
            3 => self.faces.push(Face::Triangle([face[0], face[1], face[2]])),
            4 => self
                .faces
                .push(Face::Quad([face[0], face[1], face[2], face[3]])),
            _ => return Err(ObjLoadError::InvalidFaceValue),
        }

        Ok(())
    }

    fn load_vertex(&mut self, raw_vertices: Split<&str>) -> Result<(), ObjLoadError> {
        let mut vertex = [0.0; 3];

        for (i, elem) in raw_vertices.enumerate() {
            match elem.parse::<f32>() {
                Ok(val) => vertex[i] = val,
                Err(_) => return Err(ObjLoadError::InvalidPositionValue),
            }
        }
        self.positions.push(vertex);

        Ok(())
    }

    fn load_texture_coord(&mut self, raw_coord: Split<&str>) -> Result<(), ObjLoadError> {
        let mut texture_coord = [0.0; 2];

        for (i, elem) in raw_coord.enumerate() {
            match elem.parse::<f32>() {
                Ok(val) => texture_coord[i] = val,
                Err(_) => return Err(ObjLoadError::InvalidTextureCoordValue),
            }
        }

        self.texture_coords.push(texture_coord);

        Ok(())
    }

    fn load_normal(&mut self, raw_normal: Split<&str>) -> Result<(), ObjLoadError> {
        let mut normal = [0.0; 3];

        for (i, elem) in raw_normal.enumerate() {
            match elem.parse::<f32>() {
                Ok(val) => normal[i] = val,
                Err(_) => return Err(ObjLoadError::InvalidNormalValue),
            }
        }
        self.normals.push(normal);

        Ok(())
    }
}

pub enum ObjLoadError {
    InvalidPositionValue,
    InvalidTextureCoordValue,
    InvalidNormalValue,
    InvalidFaceValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct VertexIndices {
    position: usize,
    texture_coord: usize,
    normal: usize,
}

impl Default for VertexIndices {
    fn default() -> Self {
        Self {
            position: Default::default(),
            texture_coord: Default::default(),
            normal: Default::default(),
        }
    }
}

#[derive(Debug)]
enum Face {
    Point([VertexIndices; 1]),
    Line([VertexIndices; 2]),
    Triangle([VertexIndices; 3]),
    Quad([VertexIndices; 4]),
}
