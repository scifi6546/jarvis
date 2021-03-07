mod backend;
use backend::Backend;
use jarvis::prelude::*;
use wasm_bindgen::JsCast;
pub struct WebGL {
    backend: backend::Backend,
}
impl WebGL {
    pub fn new() -> Self {
        let backend = Backend::new().expect("failed to create backend");

        Self { backend }
    }
}
impl Plugin for WebGL {
    fn setup(&mut self, _world: &mut GameState) {
        info!("created game");
    }
    fn on_frame(&mut self, _world: &mut GameState) {
        info!("rendering");
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
