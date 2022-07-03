#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(debug_assertions)]
pub mod debug;

pub mod assets;
pub mod block_type;
pub mod camera;
pub mod collisions;
pub mod enemy;
pub mod fireball;
pub mod health;
pub mod hud;
pub mod pepper;
pub mod player;
pub mod terrain;
pub mod utils;
