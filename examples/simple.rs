use legion::prelude::*;
use chez::{Application, Transform};

struct SimpleApp;

impl Application for SimpleApp {
    fn update(&self) {}
}

fn main() {
    let app = SimpleApp {};
    let universe = Universe::new();
    let mut world = universe.create_world();
    world.insert((), vec!([Transform::new(),]));
    app.start();
}