use reify::engine;
use reify::system::NativeSystem;

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    engine::run::<_, _, NativeSystem>(std::env::args())
        .expect("Error executing engine");
}