pub mod tiles;
pub mod world;

use tiles::STONE;
use world::{generate_map, World};

pub fn generate_world() {
    let mut world = World {
        map: generate_map(50, 10),
    };
    world.set_tile(2, 1, STONE);

    println!("Hello, world!\n{}", world.render_ascii_map());
}
