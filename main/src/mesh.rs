// A mesh is a digital "spider web" made of:
// 1 Vertex: a single dot in 3D space (X, Y, Z coordinates + extra data like color)
// 2 Index / Triangles: one line that connects 3 vertices directly to form a flat surface (Face)
// Note GPU only cares about vertices and triangles because they far more faster than others
use wgpu::util::DeviceExt;


// repr keeps the struct in order
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)] 
pub struct Vertex {
    pub position: [f32; 3], // position takes 3 floating point numbers
    pub color: [f32; 3],
}

// const is a value that is created at a compile time and cannot be changed at runtime
// define the dots of the triangle
pub const TRIANGLE_VERTICES: &[Vertex] = & [
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [1.0, 0.0, 3.0]
    },
    Vertex {
        position: [-0.0, 0.5, 0.0],
        color: [0.0, 2.0, 1.0]
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [4.0, 2.0, 0.0]
    }
];

// use a function to create the vertex buffer for the gpu
pub fn create_triangle_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Vertex Buffer"),
        contents: bytemuck::cast_slice(TRIANGLE_VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    })
}