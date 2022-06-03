//! A simple .obj model loading module

use std::{collections::HashMap, str::{Split, FromStr}};

use crate::model::{Material, Mesh, ModelVertex, MaterialIllumination};

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

struct ModelLoader {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
}

fn load_mtl(raw_mtl: &str) -> Result<Vec<Material>, ()> {
    let mut materials = vec!();
    let mut material = Material::default();

    for line in raw_mtl.lines() {
        let mut elements = line.split(" ");
        match elements.next() {
            Some(key) => match key {
                "Ns" => {
                    match load_num(elements.next()) {
                        Ok(f) => material.specular_exponent = f,
                        Err(_) => todo!(),
                    }
                },
                "Ka" => material.ambient_color = load_n_float::<3>(&mut elements),
                "Kd" => material.diffuse_color = load_n_float::<3>(&mut elements),
                "Ks" => material.specular_color = load_n_float::<3>(&mut elements),
                "Ke" => material.emissive_color = load_n_float::<3>(&mut elements),
                "Ni" => {
                    match load_num(elements.next()) {
                        Ok(f) => material.optical_density = f,
                        Err(_) => return Err(()),
                    }
                },
                "d" => {
                    match load_num(elements.next()) {
                        Ok(f) => material.opacity = f,
                        Err(_) => return Err(()),
                    }
                },
                "Tr" => {
                    match load_num::<f32>(elements.next()) {
                        Ok(f) => material.opacity = 1.0 - f,
                        Err(_) => return Err(()),
                    }
                },
                "illum" => {
                    match load_num::<u32>(elements.next()) {
                        Ok(i) => material.illumination_mode = load_illumination_mode(i),
                        Err(_) => return Err(()),
                    }
                },
                "map_Bump" => {
                    match elements.next() {
                        Some(file) => material.bump_map_file = file.to_string(),
                        None => return Err(()),
                    }
                }
                "map_Kd" => {
                    match elements.next() {
                        Some(file) => material.diffuse_texture_file = file.to_string(),
                        None => return Err(()),
                    }
                }
                _ => {} // Just ignore any unrecognised key
            },
            None => {}
        }
    }

    Ok(materials)
}

fn load_num<T: FromStr>(raw_num: Option<&str>) -> Result<T, ()> {
    match raw_num {
        Some(raw_num) => {
            let num = raw_num.parse::<T>();
            match num {
                Ok(num) => Ok(num),
                Err(_) => Err(()),
            }
        }
        None => Err(()),
    }
}

fn load_n_float<const N: usize>(raw_n_float: &mut Split<&str>) -> [f32; N] {
    let mut n_float = [0.0; N];

    for i in 0..N {
        let raw_float = raw_n_float.next();
        match raw_float {
            Some(raw_float) => n_float[i] = raw_float.parse::<f32>().unwrap(), // TODO: Handle the result here,
            None => todo!(), // TODO: This should be a result
        }
    }

    n_float
}

fn load_illumination_mode(mode: u32) -> Option<MaterialIllumination> {
 match mode {
    0 => Some(MaterialIllumination::ColorAmbientOff),
    1 => Some(MaterialIllumination::ColorAmbientOn),
    2 => Some(MaterialIllumination::Highlight),
    3 => Some(MaterialIllumination::ReflectionRayTrace),
    4 => Some(MaterialIllumination::TransparencyGlassRayTrace),
    5 => Some(MaterialIllumination::ReflectionFresnelRayTrace),
    6 => Some(MaterialIllumination::TransparencyRefractionRayTrace),
    7 => Some(MaterialIllumination::TransparencyFresnelRayTrace),
    8 => Some(MaterialIllumination::Reflection),
    9 => Some(MaterialIllumination::TransparencyGlass),
    10 => Some(MaterialIllumination::CastShadows),
    _ => None,
 }
}

#[derive(Default)]
struct MeshLoader {
    faces: Vec<Face>,
    positions: Vec<[f32; 3]>,
    texture_coords: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
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
            Face::Quad(vertex_indices) => {
                // Split the quad into 2 triangles - With vertices 0, 1, 2
                for vi in &vertex_indices[0..3] {
                    self.export_vertex(vi, mesh, vertex_map);
                }

                // & vertices 1, 2, 3
                self.export_vertex(&vertex_indices[0], mesh, vertex_map);
                self.export_vertex(&vertex_indices[2], mesh, vertex_map);
                self.export_vertex(&vertex_indices[3], mesh, vertex_map);
            }
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

pub enum ObjLoadError {
    InvalidPositionValue,
    InvalidTextureCoordValue,
    InvalidNormalValue,
    InvalidFaceValue,
}
