use jarvis::prelude::*;
pub struct Game {}
#[derive(Clone, Debug, PartialEq)]
struct Terrain {
    heights: Grid<f32>,
}
impl Terrain {
    pub fn flat(dimensions: Vector2<usize>, height: f32) -> Self {
        let data = (0..dimensions.x * dimensions.y)
            .map(|_| height.clone())
            .collect();
        let heights = Grid::new(data, dimensions);
        Self { heights }
    }
}
impl Plugin for Game {
    fn setup(&mut self, world: &mut GameState) {
        world
            .ecs()
            .push((Terrain::flat(Vector2::new(100, 100), 0.0), 0usize));
    }
    fn on_frame(&mut self, _world: &mut GameState) {}
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
