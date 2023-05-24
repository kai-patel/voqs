#![allow(dead_code)]
use crate::gen_buffers;

const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    blk: [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    elements: u32,
    changed: bool,
    vbo: wgpu::Buffer,
}

impl Default for Chunk {
    fn default() -> Self {
        let vbo = gen_buffers(1).remove(0);

        Self {
            blk: Default::default(),
            elements: 0,
            changed: false,
            vbo,
        }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Chunk::default()
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u8 {
        self.blk[x][y][z]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, kind: u8) {
        self.blk[x][y][z] = kind;
        self.changed = true;
    }

    pub fn update(&mut self) {
        self.changed = false;
        todo!("Fill in VBO");
    }

    pub fn render(&mut self) {
        if self.changed {
            self.update();
            todo!("Render the VBO");
        }
    }
}
