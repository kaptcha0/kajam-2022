#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
pub mod polyfill;

#[cfg(debug_assertions)]
pub mod debug;

pub mod assets;
pub mod block_type;
pub mod fireball;
pub mod health;
pub mod player;
pub mod terrain;
pub mod utils;
