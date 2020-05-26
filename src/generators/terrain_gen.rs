use crate::{tiles::STONE, world::World};

pub struct TerrainOptions {
    pub sea_level: usize,
    pub hole_offset: usize,
    pub mountain_height: usize,
}

pub fn generate_terrain(world: &mut World, options: TerrainOptions) {
    fill_underworld(world, &options);
    generate_mountains(world, &options);
}

fn fill_underworld(world: &mut World, options: &TerrainOptions) {
    // Fill everything under the lowest point of overworld
    world.fill_rectangle(
        0,
        world.height - options.sea_level + options.hole_offset,
        world.width,
        options.sea_level - options.hole_offset,
        STONE,
    )
}

fn generate_mountains(world: &mut World, options: &TerrainOptions) {
    let max_height = options.mountain_height + options.hole_offset;
    for x in 0..world.width {
        let perlin = world.perlin(x, 0, 40.0);
        let height = (perlin * max_height as f64).round() as usize;
        world.fill_rectangle(x, world.height - height, 1, height, STONE);
    }
}
