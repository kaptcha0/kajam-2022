use bevy::prelude::*;
use bevy::render::mesh::{Mesh, PrimitiveTopology};

pub const BLOCK_SIZE: f32 = 0.1;
pub const BLOCK_HEIGHT: f32 = 0.01;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlockType {
    Flat,
    Uphill,
    Downhill,
}

impl BlockType {
    pub fn to_sprite(&self) -> SpriteBundle {
        let sprite = Sprite {
            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_HEIGHT)),
            ..Default::default()
        };

        return match self {
            BlockType::Flat => SpriteBundle {
                sprite,
                transform: Transform::default(),
                ..Default::default()
            },
            BlockType::Uphill => {
                let angle = 45.0_f32.to_radians();

                SpriteBundle {
                    sprite,
                    transform: Transform::default()
                        .with_rotation(Quat::from_axis_angle(Vec3::Z, angle)),
                    ..Default::default()
                }
            }
            BlockType::Downhill => {
                let angle = 315.0_f32.to_radians();

                SpriteBundle {
                    sprite,
                    transform: Transform::default()
                        .with_rotation(Quat::from_axis_angle(Vec3::Z, angle)),
                    ..Default::default()
                }
            }
        };
    }

    fn to_mesh(&self) -> Mesh {
        return match self {
            BlockType::Flat => {
                let vertices = [
                    ([0.5, 0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
                    ([-0.5, 0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
                ];
                let mut positions = Vec::<[f32; 3]>::new();
                let mut normals = Vec::<[f32; 3]>::new();
                let mut uvs = Vec::<[f32; 2]>::new();
                for (position, normal, uv) in &vertices {
                    positions.push(*position);
                    normals.push(*normal);
                    uvs.push(*uv);
                }

                let mut mesh = Mesh::new(PrimitiveTopology::LineList);
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                mesh
            }
            BlockType::Uphill => {
                let vertices = [
                    ([-0.5, -0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
                    ([0.5, 0.5, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
                ];

                let mut positions = Vec::<[f32; 3]>::new();
                let mut normals = Vec::<[f32; 3]>::new();
                let mut uvs = Vec::<[f32; 2]>::new();
                for (position, normal, uv) in &vertices {
                    positions.push(*position);
                    normals.push(*normal);
                    uvs.push(*uv);
                }

                let mut mesh = Mesh::new(PrimitiveTopology::LineList);
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                mesh
            }
            BlockType::Downhill => {
                let vertices = [
                    ([-0.5, 1.5, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
                    ([0.5, 0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
                ];

                let mut positions = Vec::<[f32; 3]>::new();
                let mut normals = Vec::<[f32; 3]>::new();
                let mut uvs = Vec::<[f32; 2]>::new();
                for (position, normal, uv) in &vertices {
                    positions.push(*position);
                    normals.push(*normal);
                    uvs.push(*uv);
                }

                let mut mesh = Mesh::new(PrimitiveTopology::LineList);
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                mesh
            }
        };
    }
}

impl From<BlockType> for SpriteBundle {
    fn from(block: BlockType) -> Self {
        block.to_sprite()
    }
}

impl From<BlockType> for Mesh {
    fn from(block: BlockType) -> Self {
        block.to_mesh()
    }
}
