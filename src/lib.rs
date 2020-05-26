pub mod generators;
pub mod tiles;
pub mod world;

use generators::terrain_gen::{generate_terrain, TerrainOptions};
use world::World;

pub fn generate_world() {
    let mut world = World::new(50, 15);
    generate_terrain(
        &mut world,
        TerrainOptions {
            sea_level: 5,
            hole_offset: 3,
        },
    );

    println!("Hello, world!\n{}", world.render_ascii_map());
}
