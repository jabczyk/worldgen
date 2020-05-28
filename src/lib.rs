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

    println!("{}", world.render_ascii_map());
}
