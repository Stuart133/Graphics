use std::ops::Range;

use cgmath::{num_traits::Pow, Matrix4, Vector4};
use wgpu::*;

use crate::model::Vertex;

pub trait Curve {
    fn to_vertices(&self, range: Range<f32>, steps: u32) -> Vec<CurveVertex>;
}

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

impl Curve for BezierCurve {
    fn to_vertices(&self, range: Range<f32>, steps: u32) -> Vec<CurveVertex> {
        let mut curve = vec![];

        // Add the control points first
        curve.push(CurveVertex {
            position: [self.control_points.x.x, self.control_points.x.y, self.control_points.x.z]
        });
        curve.push(CurveVertex {
            position: [self.control_points.y.x, self.control_points.y.y, self.control_points.y.z]
        });
        curve.push(CurveVertex {
            position: [self.control_points.z.x, self.control_points.z.y, self.control_points.z.z]
        });
        curve.push(CurveVertex {
            position: [self.control_points.w.x, self.control_points.w.y, self.control_points.w.z]
        });

        // firstly create the cubic function in the canonical basis
        let canonical = |t: f32| -> Vector4<f32> {
            self.control_points * BEZIER_SPLINE * Vector4::new(1.0, t, t.pow(2.0), t.pow(3.0))
        };

        for i in 0..steps + 1 {
            let t = (range.end - range.start) / steps as f32 * i as f32;

            let point = canonical(t);
            curve.push(CurveVertex {
                position: [point.x, point.y, point.z],
            })
        }

        println!("{:?}", curve.len());
        curve
    }

    // TODO: Try De Casteljau's algorithm for rendering the points
}

const BEZIER_SPLINE: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, -3.0, 3.0, 0.0, 0.0, 3.0, -6.0, 3.0, 0.0, 1.0, 3.0, -3.0, 1.0,
);
