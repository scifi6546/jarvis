use legion::*;
use nalgebra::Vector2;
mod mesh;
mod texture;
pub trait Plugin {
    /// Makes sure the game is ready for use
    fn setup(&mut self, game: &mut GameState);
    /// Updates Every frame
    fn on_frame(&mut self, game: &mut GameState);
}
pub struct GameState {
    entities: World,
}
impl GameState {
    pub fn ecs(&mut self) -> &mut World {
        &mut self.entities
    }
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            entities: World::default(),
        }
    }
}
pub struct AssetHandle {
    index: generational_arena::Index,
}
pub struct AssetManager<T> {
    items: generational_arena::Arena<T>,
    to_process: Vec<generational_arena::Index>,
}
impl<T> AssetManager<T> {
    pub fn insert(&mut self, asset: T) -> AssetHandle {
        let index = self.items.insert(asset);
        self.to_process.push(index.clone());
        AssetHandle { index }
    }
    pub fn get(&self, handle: AssetHandle) -> &T {
        self.items.get(handle.index).unwrap()
    }
    pub fn handles_to_process(&self) {}
}
impl<T> Default for AssetManager<T> {
    fn default() -> AssetManager<T> {
        AssetManager {
            items: generational_arena::Arena::default(),
            to_process: Vec::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Grid<T> {
    data: Vec<T>,
    dimensions: Vector2<usize>,
}
impl<T> Grid<T> {
    /// Creates grid, expects data to be in form```
    /// [(0,0),(1,0),(2,0),
    /// ,(1,0),(1,1),(1,2)]
    /// ````
    pub fn new(data: Vec<T>, dimensions: Vector2<usize>) -> Self {
        Grid { data, dimensions }
    }
    pub fn get(&self, location: Vector2<usize>) -> &T {
        &self.data[location.y * self.dimensions.x + location.x]
    }
}
pub struct Texture {}
pub struct Model {}
pub mod prelude {
    pub use super::mesh::Mesh;
    pub use super::texture::Texture;
    pub use super::AssetManager;
    pub use super::GameState;
    pub use super::Grid;
    pub use super::Plugin;
    pub use log::{debug, error, info, Level as LogLevel};
    pub use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
