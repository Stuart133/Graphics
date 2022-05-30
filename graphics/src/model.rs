use cgmath::Vector3;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    *,
};

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

/// A generalized mesh represting a portion of a 3d model
///
/// This is not necessarily in a form ready for consumption by the GPU
/// but is a more direct representation of the original mesh data
#[derive(Debug, Default)]
pub struct Mesh {
    /// Name of the mesh
    pub name: String,

    /// Vector of mesh vertices
    pub vertices: Vec<ModelVertex>,

    /// Vector of vertex indices
    pub indices: Vec<u32>,
}

/// A generalized material to be applied to a mesh
///
/// This is not necessarily in a form ready for consumption by the GPU
#[derive(Debug)]
pub struct Material {
    /// Name of the material
    pub name: String,

    /// Specular exponent of the material, controlling object glossiness
    pub specular_exponent: f32,

    /// Specular color of the material
    pub specular_color: Vector3<f32>,

    /// Ambient color of the material
    pub ambient_color: Vector3<f32>,

    /// Diffuse color of the material
    pub diffuse_color: Vector3<f32>,

    /// Emissive color of the material
    pub emissive_color: Vector3<f32>,

    /// Index of refraction of the material
    pub optical_density: f32,

    /// Opactiy of the material. The inverse of transparency
    pub opacity: f32,

    /// Illumintation mode of the material. Often now not specified
    pub illumination_mode: Option<MaterialIllumination>,

    /// Path to normal map file
    pub bump_map_file: String,

    /// Path to diffuse texutre file
    pub diffuse_texture_file: String,
}

/// Material Illumintaion Modes
///
/// See https://en.wikipedia.org/wiki/Wavefront_.obj_file#Reference_materials
/// for more details
#[derive(Debug)]
pub enum MaterialIllumination {
    ColorAmbientOff = 0,
    ColorAmbiantOn = 1,
    Highlight = 2,
    ReflectionRayTrace = 3,
    TransparencyGlassRayTrace = 4,
    ReflectionFresnelRayTrace = 5,
    TransparencyRefractionRayTrace = 6,
    TransparencyFresnelRayTrace = 7,
    Reflection = 8,
    TransparencyGlass = 9,
    CastShadows = 10,
}

// TODO: Pass through errors better
#[derive(Debug)]
pub enum ModelLoadError {
    InvalidModel,
}

#[derive(Debug)]
pub struct Model<'a> {
    pub meshes: Vec<GpuMesh>, // TODO: Probably move vertex/index buffer to here
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
            vertex_count: mesh.indices.len() as u32, // TODO: Cast more sensibly
        }
    }
}

impl<'a> Model<'a> {
    pub fn from_str(
        raw_model: &str,
        device: &Device,
        label: Option<&'a str>,
    ) -> Result<Model<'a>, ModelLoadError> {
        match crate::obj::from_str(raw_model) {
            Ok(mesh) => Ok(Model {
                meshes: vec![GpuMesh::from_mesh(mesh, device)],
                label,
            }),
            Err(_) => Err(ModelLoadError::InvalidModel),
        }
    }
}
