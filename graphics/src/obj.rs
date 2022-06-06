//! A simple .obj model loading module
// TODO: Write some tests for this module

use std::{
    collections::HashMap,
    path::Path,
    str::{FromStr, Split},
};

use crate::model::{Material, MaterialIllumination, Mesh, Model, ModelVertex};

pub fn load_model(file: &Path) -> Result<Model, ObjLoadError> {
    let raw_model = match std::fs::read_to_string(file) {
        Ok(str) => str,
        Err(err) => return Err(ObjLoadError::FileLoadError(err)),
    };

    let mut loader = ModelLoader::default();

    let mut prev = 0;

    let lines: Vec<&str> = raw_model.lines().collect();
    for i in 0..lines.len() {
        let mut elements = lines[i].split(" ");
        match elements.next() {
            Some(key) => match key {
                "mtllib" => match elements.next() {
                    Some(mtl_file) => {
                        match std::fs::read_to_string(
                            file.parent().unwrap().join(Path::new(mtl_file)),
                        ) {
                            Ok(raw_mtl) => match loader.load_mtl(raw_mtl.as_str()) {
                                Some(err) => return Err(err),
                                None => {}
                            },
                            Err(err) => return Err(ObjLoadError::FileLoadError(err)),
                        }
                    }
                    None => return Err(ObjLoadError::InvalidMaterialLib),
                },
                "o" | "g" => {
                    if prev == 0 {
                        // First mesh encountered
                        prev = i;
                    } else {
                        match loader.load_mesh(&lines[prev..i]) {
                            Some(err) => return Err(err),
                            None => {}
                        };
                        prev = i;
                        loader.current_material = String::default();
                        loader.current_faces.clear();
                    }
                }
                _ => {} // Just ignore any unrecognised key
            },
            None => todo!(),
        }
    }

    // Load final mesh
    match loader.load_mesh(&lines[prev..]) {
        Some(err) => return Err(err),
        None => {}
    }

    Ok(Model {
        meshes: loader.meshes,
        materials: loader.materials,
    })
}

#[derive(Default)]
struct ModelLoader {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
    material_map: HashMap<String, usize>,
    positions: Vec<[f32; 3]>,
    texture_coords: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
    current_faces: Vec<Face>,
    current_material: String,
}

impl ModelLoader {
    fn load_mesh(&mut self, raw_mesh: &[&str]) -> Option<ObjLoadError> {
        for line in raw_mesh.iter() {
            let mut elements = line.split(" ");
            match elements.next() {
                Some(key) => match key {
                    "v" => match self.load_vertex(elements) {
                        Ok(_) => {}
                        Err(err) => return Some(err),
                    },
                    "vt" => match self.load_texture_coord(elements) {
                        Ok(_) => {}
                        Err(err) => return Some(err),
                    },
                    "vn" => match self.load_normal(elements) {
                        Ok(_) => {}
                        Err(err) => return Some(err),
                    },
                    "f" => match self.load_face(elements) {
                        Ok(_) => {}
                        Err(err) => return Some(err),
                    },
                    "usemtl" => match elements.next() {
                        Some(mtl_name) => self.current_material = mtl_name.to_string(),
                        None => return Some(ObjLoadError::InvalidMaterialName),
                    },
                    _ => {} // Just ignore any unrecognised key
                },
                None => {}
            }
        }

        // Groups/Objects can be defined with no faces, in which case there is no mesh
        if self.current_faces.len() > 0 {
            self.meshes.push(self.export_mesh());
        }

        None
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
            1 => self.current_faces.push(Face::Point([face[0]])),
            2 => self.current_faces.push(Face::Line([face[0], face[1]])),
            3 => self
                .current_faces
                .push(Face::Triangle([face[0], face[1], face[2]])),
            4 => self
                .current_faces
                .push(Face::Quad([face[0], face[1], face[2], face[3]])),
            _ => return Err(ObjLoadError::InvalidFaceValue),
        }

        Ok(())
    }

    // TODO - Use load N float in these methods
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

    // TODO - Use load N float in these methods
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

    // TODO - Use load N float in these methods
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

    fn export_mesh(&self) -> Mesh {
        let mut mesh = Mesh::default();
        let mut vertex_map = Default::default();

        for face in self.current_faces.iter() {
            self.export_face(face, &mut mesh, &mut vertex_map);
        }

        mesh.material = self.material_map.get(&self.current_material).cloned();

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

    // TODO - Swap load material into the impl and this to the top level
    fn load_mtl(&mut self, raw_mtl: &str) -> Option<ObjLoadError> {
        let mut prev = 0;
        let mut current_material_name = "".to_string();

        let lines: Vec<&str> = raw_mtl.lines().collect();
        for i in 0..lines.len() {
            let mut elements = lines[i].split(" ");
            match elements.next() {
                Some(key) => {
                    match key {
                        "newmtl" => {
                            if prev == 0 {
                                // First mtl encountered
                                current_material_name = elements.next().unwrap().to_string();
                                prev = i;
                            } else {
                                match load_material(&lines[prev..i]) {
                                    Ok(mat) => self.push_material(mat, current_material_name),
                                    Err(_) => return Some(ObjLoadError::InvalidMaterialLib),
                                };
                                prev = i;
                                current_material_name = elements.next().unwrap().to_string();
                            }
                        }
                        _ => {}
                    };
                }
                None => {}
            }
        }

        // Load final mtl
        match load_material(&lines[prev..]) {
            Ok(mat) => self.push_material(mat, current_material_name),
            Err(_) => return Some(ObjLoadError::InvalidMaterialLib),
        };

        None
    }

    fn push_material(&mut self, material: Material, material_name: String) {
        self.material_map
            .insert(material_name, self.materials.len());
        self.materials.push(material);
    }
}

fn load_material(raw_material: &[&str]) -> Result<Material, ()> {
    let mut material = Material::default();

    for line in raw_material.iter() {
        let mut elements = line.split(" ");
        match elements.next() {
            Some(key) => match key {
                "Ns" => match load_num(elements.next()) {
                    Ok(f) => material.specular_exponent = f,
                    Err(_) => todo!(),
                },
                "Ka" => material.ambient_color = load_n_float::<3>(&mut elements),
                "Kd" => material.diffuse_color = load_n_float::<3>(&mut elements),
                "Ks" => material.specular_color = load_n_float::<3>(&mut elements),
                "Ke" => material.emissive_color = load_n_float::<3>(&mut elements),
                "Ni" => match load_num(elements.next()) {
                    Ok(f) => material.optical_density = f,
                    Err(_) => return Err(()),
                },
                "d" => match load_num(elements.next()) {
                    Ok(f) => material.opacity = f,
                    Err(_) => return Err(()),
                },
                "Tr" => match load_num::<f32>(elements.next()) {
                    Ok(f) => material.opacity = 1.0 - f,
                    Err(_) => return Err(()),
                },
                "illum" => match load_num::<u32>(elements.next()) {
                    Ok(i) => material.illumination_mode = load_illumination_mode(i),
                    Err(_) => return Err(()),
                },
                "map_Bump" => match elements.next() {
                    Some(file) => material.bump_map_file = file.to_string(),
                    None => return Err(()),
                },
                "map_Kd" => match elements.next() {
                    Some(file) => material.diffuse_texture_file = file.to_string(),
                    None => return Err(()),
                },
                _ => {} // Just ignore any unrecognised key
            },
            None => {}
        }
    }

    Ok(material)
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

#[derive(Debug)]
pub enum ObjLoadError {
    FileLoadError(std::io::Error),
    InvalidPositionValue,
    InvalidTextureCoordValue,
    InvalidNormalValue,
    InvalidFaceValue,
    InvalidMaterialName,
    InvalidMaterialLib,
}
