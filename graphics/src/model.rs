use wgpu::{*, util::{BufferInitDescriptor, DeviceExt}};

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
    pub meshes: Vec<GpuMesh>,   // TODO: Probably move vertex/index buffer to here
    pub label: Option<&'a str>,
}

/// Mesh representation for sending to the GPU
#[derive(Debug)]
pub struct GpuMesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub vertex_count: u32,
}

impl GpuMesh {
    fn from_mesh(mesh: Mesh, device: &Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("Vertex Buffer")),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("Index Buffer")),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: BufferUsages::INDEX,
        });
        
        GpuMesh {
            vertex_buffer,
            index_buffer,
            vertex_count: mesh.indices.len() as u32,    // TODO: Cast more sensibly
        }
    }
}

impl<'a> Model<'a> {
    pub fn from_str(raw_model: &str, device: &Device, label: Option<&'a str>) -> Result<Model<'a>, ModelLoadError> {
        match crate::obj::from_str(raw_model) {
            Ok(mesh) => Ok(Model {
                meshes: vec![GpuMesh::from_mesh(mesh, device)],
                label,
            }),
            Err(_) => Err(ModelLoadError::InvalidModel),
        }
    }
}
