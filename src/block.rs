use std::collections::HashMap;
use bevy::prelude::Handle;
use crate::TextureAtlas;

pub struct Block {
    pub x: u32,
    pub y: u32,
    pub id: u32
}

pub struct BlockTexturesIndex {
    pub brick: usize
}

pub struct BlockTexture {
    pub textures: HashMap<u32, usize>
}

pub struct BlockAtlas {
    pub handle: Handle<TextureAtlas>
}

pub fn create(data: Vec<u32>) -> Block {
    let block_id = data.get(0).unwrap();
    let x = data.get(1).unwrap();
    let y = data.get(2).unwrap();
    match block_id {
        0 => Block {x: *x, y: *y, id: 0},
        _ => Block {x: 0, y: 0, id: 0}
    }
}