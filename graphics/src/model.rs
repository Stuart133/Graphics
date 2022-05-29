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

impl ModelVertex {
    pub fn new(position: [f32; 3], texture_coords: [f32; 2], normal: [f32; 3]) -> ModelVertex {
        ModelVertex {
            position,
            texture_coords,
            normal,
        }
    }
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

// TODO: Pass through errors better
#[derive(Debug)]
pub enum ModelLoadError {
    InvalidModel,
}

#[derive(Debug)]
pub struct Model<'a> {
    pub meshes: Vec<Mesh>,
    pub label: Option<&'a str>,
}

impl<'a> Model<'a> {
    pub fn from_str(raw_model: &str, label: Option<&'a str>) -> Result<Model<'a>, ModelLoadError> {
        match crate::obj::from_str(raw_model) {
            Ok(mesh) => Ok(Model {
                meshes: vec![mesh],
                label,
            }),
            Err(_) => Err(ModelLoadError::InvalidModel),
        }
    }
}
