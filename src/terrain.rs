use bevy::prelude::*;
use heron::prelude::*;
use noise::NoiseFn;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    block_type::{BlockType, BLOCK_HEIGHT, BLOCK_SIZE},
    utils::Layers,
};

pub const MAP_LEN: u32 = 500;
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Terrain>()
            .add_startup_system(generate_terrain);
    }
}

#[derive(Default)]
pub struct Terrain(Vec<Entity>);

fn generate_terrain(mut commands: Commands, mut terrain: ResMut<Terrain>) {
    let heights = generate_heightmap(MAP_LEN);
    let raw_blocks = heightmap_to_blocks(heights);
    let blocks = process_blocks(raw_blocks);

    let mut y = 0.0;
    let mut x = -BLOCK_SIZE;

    for (i, (block, new_y)) in blocks.iter().enumerate() {
        let color = color_block(i, blocks.len());

        y += new_y;
        x += BLOCK_SIZE;

        let (x_offset, y_offset) = match block {
            BlockType::Flat => {
                if i == 0 {
                    (x, y)
                } else {
                    let (prev, _) = blocks[i - 1];

                    let x = match prev {
                        BlockType::Flat => x,
                        BlockType::Uphill | BlockType::Downhill => x - 0.02,
                    };

                    (x, y)
                }
            }
            _ => (x - 0.02, (y - new_y) + (new_y / 2.0)),
        };

        x = x_offset;

        let mut sprite = block.to_sprite();
        sprite.sprite.color = color;
        sprite.transform = sprite
            .transform
            .with_translation(Vec3::new(x_offset, y_offset, 900.0));

        let entity = commands
            .spawn_bundle(sprite)
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(BLOCK_SIZE / 2.0, BLOCK_HEIGHT / 2.0, 0.0),
                border_radius: None,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layers::Level)
                    .with_masks(&[Layers::Player, Layers::Enemy]),
            )
            .id();

        terrain.0.push(entity)
    }

    commands
        .spawn()
        .insert(Name::new("Level"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&terrain.0);
}

fn process_blocks(blocks: Vec<BlockType>) -> Vec<(BlockType, f32)> {
    blocks
        .iter()
        .enumerate()
        .map(|(i, block)| {
            if i == 0 {
                (BlockType::Flat, 0.0)
            } else {
                process_block(*block, blocks[i - 1])
            }
        })
        .collect()
}

fn process_block(mut block: BlockType, prev: BlockType) -> (BlockType, f32) {
    block = match block {
        BlockType::Flat => block,
        BlockType::Uphill | BlockType::Downhill => match prev {
            BlockType::Flat => block,
            BlockType::Uphill | BlockType::Downhill => BlockType::Flat,
        },
    };

    let angled_offset = BLOCK_SIZE * 45.0_f32.to_radians().sin();

    let y = match block {
        BlockType::Uphill => angled_offset - 0.005,
        BlockType::Downhill => -angled_offset + 0.005,
        BlockType::Flat => 0.0,
    };

    (block, y)
}

fn color_block(i: usize, len: usize) -> Color {
    let percentage = i as f32 / len as f32;

    let red = Vec3::new(255.0, 6.0, 0.0);
    let blue = Vec3::new(0.0, 0.0, 255.0);
    let cyan = Vec3::new(0.0, 255.0, 255.0);

    let color_vec = cyan.lerp(blue.lerp(red, percentage), percentage);

    // Normalize lerped values between 0 and 1
    Color::Rgba {
        red: color_vec.x / 255.0,
        green: color_vec.y / 255.0,
        blue: color_vec.z / 255.0,
        alpha: 1.0,
    }
}

fn heightmap_to_blocks(heights: Vec<f32>) -> Vec<BlockType> {
    heights
        .iter()
        .enumerate()
        .map(|(index, height)| {
            if (index == 0) || (index == heights.len() - 1) {
                return BlockType::Flat;
            }

            let prev = &heights[index - 1];
            let delta = height - prev;

            if delta > 0.3 {
                return BlockType::Uphill;
            } else if delta < -0.4 {
                return BlockType::Downhill;
            } else {
                return BlockType::Flat;
            }
        })
        .collect()
}

fn generate_heightmap(length: u32) -> Vec<f32> {
    debug!("generating terrain with a length of {}", length);
    let mut heights = Vec::new();
    let noise_fn = noise::SuperSimplex::new();
    let rand = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();

    for i in 0..length {
        let val = (i as f64).sin();
        let height = noise_fn.get([
            (rand as f64 / val),
            ((val.tan() as u32) >> (val.cosh() as u32)) as f64,
            (rand ^ (val as u128)) as f64,
        ]);

        heights.push(height as f32);
    }

    debug!("generatied terrain");

    heights
}
