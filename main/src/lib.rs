use jarvis::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() -> Game {
    console_log::init_with_level(LogLevel::Debug).expect("failed to init log");
    info!("hello world");
    let game = GameState::default();
    let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(game::Game {})];
    Game { game, plugins }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
#[wasm_bindgen]
pub struct Game {
    game: GameState,
    plugins: Vec<Box<dyn Plugin>>,
}

#[wasm_bindgen]
impl Game {
    pub fn startup(&mut self) {
        for plug in self.plugins.iter_mut() {
            plug.setup(&mut self.game);
        }
    }
    pub fn run_frame(&mut self) {
        for plug in self.plugins.iter_mut() {
            plug.on_frame(&mut self.game);
        }
    }
}
