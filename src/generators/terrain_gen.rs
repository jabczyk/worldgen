use crate::{tiles::STONE, world::World};

pub struct TerrainOptions {
    pub sea_level: usize,
    pub hole_offset: usize,
}

pub fn generate_terrain(world: &mut World, options: TerrainOptions) {
    fill_underworld(world, options);
}

fn fill_underworld(world: &mut World, options: TerrainOptions) {
    // Fill everything under the lowest point of overworld
    world.fill_rectangle(
        0,
        world.height - options.sea_level + options.hole_offset,
        world.width,
        options.sea_level - options.hole_offset,
        STONE,
    )
}
