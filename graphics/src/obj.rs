//! A simple .obj mesh loader

use std::str::Split;

pub struct Mesh {
    /// Vector of vertex positions
    pub positions: Vec<[f32; 3]>,

    /// Vector of texture coordinates
    pub texture_coords: Vec<[f32; 2]>,

    /// Vector of vertex normals vectors
    pub normals: Vec<[f32; 3]>,

    pub indices: Vec<usize>,
}

impl Mesh {
    pub fn from_str(raw_mesh: &str) -> Result<Mesh, ObjLoadError> {
        let loader = MeshLoader::default();

        for line in raw_mesh.lines() {
            let elements = line.split(" ");
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
                    "vn" => match loader.load_texture_coord(elements) {
                        Ok(_) => {}
                        Err(err) => return Err(err),
                    },
                    "f" => match loader.load_face(elements) {
                      Ok(_) => {}
                      Err(err) => return Err(err),
                    }
                    _ => {} // Just ignore any unrecognised key
                },
                None => {}
            }
        }

        Ok(())
    }
}

pub struct MeshLoader {
    faces: Vec<Face>,
    vertices: Vec<[f32; 3]>,
    texture_coords: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
}

impl Default for MeshLoader {
    fn default() -> Self {
        Self {
            faces: Default::default(),
            vertices: Default::default(),
            texture_coords: Default::default(),
            normals: Default::default(),
        }
    }
}

impl MeshLoader {
    fn load_face(&mut self, raw_face: Split<&str>) -> Result<(), ObjLoadError> {
        let face = vec![];

        for group in raw_face {
            let mut indices = VertexIndices::default();
            for (i, index) in group.split("/").enumerate() {
                // Value could be missing - If it is skip to next value
                if index == "" {
                    continue;
                }

                match index.parse::<usize>() {
                    Ok(index) => match i {
                        0 => indices.position = index,
                        1 => indices.texture_coord = index,
                        2 => indices.normal = index,
                        _ => return Err(ObjLoadError::InvalidFaceValue),
                    },
                    Err(err) => return Err(ObjLoadError::InvalidFaceValue),
                }

                face.push(indices);
            }
        }

        match face.len() {
          1 => self.faces.push(Face::Point(face[0])),
          2 => self.faces.push(Face::Line(face[0], face[1])),
          3 => self.faces.push(Face::Triangle(face[0], face[1], face[2])),
          4 => self.faces.push(Face::Quad(face[0], face[1], face[2], face[3])),
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
        self.vertices.push(vertex);

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
                Err(_) => return Err(ObjLoadError::InvalidPositionValue),
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

enum Face {
    Point(VertexIndices),
    Line(VertexIndices, VertexIndices),
    Triangle(VertexIndices, VertexIndices, VertexIndices),
    Quad(VertexIndices, VertexIndices, VertexIndices, VertexIndices),
}
