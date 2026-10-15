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
pub const CUBE_VERTICES: &[Vertex] = & [
    Vertex { position: [-1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] }, // Down left (RED)
    Vertex { position: [ 1.0, -1.0,  1.0], color: [0.0, 1.0, 0.0] }, // Down right (GREEN)
    Vertex { position: [ 1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] }, // Up right (BLUE)
    Vertex { position: [-1.0,  1.0,  1.0], color: [1.0, 1.0, 0.0] }, // Up left (YELLOW)
    // Takatahko (Z = -1.0)
    Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 1.0] }, // 4
    Vertex { position: [ 1.0, -1.0, -1.0], color: [0.0, 1.0, 1.0] }, // 5
    Vertex { position: [ 1.0,  1.0, -1.0], color: [1.0, 1.0, 1.0] }, // 6
    Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 0.0, 0.0] }, // 7
];

pub const CUBE_INDICES: &[u16] = & [
    0, 1, 2,  2, 3, 0,
    1, 5, 6,  6, 2, 1,
    7, 6, 5,  5, 4, 7,
    4, 0, 3,  3, 7, 4,
    4, 5, 1,  1, 0, 4,
    3, 2, 6,  6, 7, 3,
];

// use a function to create the vertex buffer for the gpu
pub fn create_cube_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Vertex Buffer"),
        contents: bytemuck::cast_slice(CUBE_VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

// create index buffer from indices
pub fn create_index_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(CUBE_INDICES),
        usage: wgpu::BufferUsages::INDEX,
    })
}