use std::ops::Range;

use cgmath::{Matrix2, Matrix4, Vector4};
use wgpu::*;

use crate::model::Vertex;

/// Curve Vertex
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CurveVertex {
    position: [f32; 3],
}

impl Vertex for CurveVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<CurveVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float32x3,
            }],
        }
    }
}

/// Bezier curve in 3D space
pub struct BezierCurve {
    pub control_points: Matrix4<f32>,
}

impl BezierCurve {
    fn to_vertices(&self, range: Range<f32>) -> Vec<CurveVertex> {
        // firstly create the cubic function in the canonical basis
        let canonical = self.control_points * BEZIER_SPLINE;
    }

    // TODO: Try De Casteljau's algorithm for rendering the points
}

const BEZIER_SPLINE: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, -3.0, 3.0, -6.0, 3.0, 0.0, 0.0, 3.0, -3.0, 0.0, 0.0, 0.0, 1.0,
);
