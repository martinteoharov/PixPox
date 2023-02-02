use rand::Rng;

pub struct Colors {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Position {
    x: u16,
    y: u16,
}

pub enum PixelClass {
    SAND,
    WATER,
    STONE,
    IRON,
}

pub struct Pixel {
    pub colors: Colors,
    pub pos: Position,
    pub class: PixelClass,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }

    pub fn gen_random_array(n: u8) -> Vec<Vertex> {
        let mut random_vertices: Vec<Vertex> = Vec::new();

        for _ in 1..n {
            let (r1, r2, r3) = (
                rand::thread_rng().gen_range(0.0..1.0),
                rand::thread_rng().gen_range(0.0..1.0),
                rand::thread_rng().gen_range(0.0..1.0),
            );

            random_vertices.push(Vertex {
                position: [r1, r2, r3],
                color: [r1, r2, r3],
            })
        }

        random_vertices
    }
}