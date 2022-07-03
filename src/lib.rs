#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
mod polyfill {
    pub fn set_panic_hook() {
        // When the `console_error_panic_hook` feature is enabled, we can call the
        // `set_panic_hook` function at least once during initialization, and then
        // we will get better error messages if our code ever panics.
        //
        // For more details see
        // https://github.com/rustwasm/console_error_panic_hook#readme
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
    }
}

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
