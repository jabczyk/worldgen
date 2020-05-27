pub mod generators;
pub mod tiles;
pub mod world;

use generators::terrain_gen::{generate_terrain, TerrainOptions};
use world::World;

pub fn generate_world() {
    let mut world = World::new(100, 15, 1);
    generate_terrain(
        &mut world,
        TerrainOptions {
            sea_level: 5,
            hole_offset: 3,
            mountain_height: 10,
        },
    );

    // Since println prints a line, no need for \n
    // Also, no real need for Hello, world :)
    println!("{}", world.render_ascii_map());
}
