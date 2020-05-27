use crate::{tiles::TileType, world::World};

pub struct TerrainOptions {
    pub sea_level: usize,
    pub hole_offset: usize,
    pub mountain_height: usize,
}

pub fn generate_terrain(world: &mut World, options: TerrainOptions) {
    fill_underworld(world, &options);
    generate_mountains(world, &options);
    generate_sea_water(world, &options);
}

fn fill_underworld(world: &mut World, options: &TerrainOptions) {
    // Fill everything under the lowest point of overworld
    world.fill_rectangle(
        0,
        world.height - options.sea_level + options.hole_offset,
        world.width,
        options.sea_level - options.hole_offset,
        TileType::Stone,
    )
}

fn generate_mountains(world: &mut World, options: &TerrainOptions) {
    let max_height = options.mountain_height + options.hole_offset;
    let horizon = world.height - (options.sea_level - options.hole_offset);

    for x in 0..world.width {
        let perlin = world.perlin(x, 0, 40.0);
        let height = (perlin * max_height as f64).round() as usize;
        let y = horizon - height;
        world.fill_rectangle(x, y, 1, height, TileType::Stone);
        replace_ground_tiles(world, options, x, y);
    }
}

fn replace_ground_tiles(world: &mut World, options: &TerrainOptions, x: usize, max_y: usize) {
    let new_tile = if max_y < world.height - options.sea_level {
        TileType::Dirt
    } else {
        TileType::Sand
    };
    let tiles_to_change = 2;
    world.fill_rectangle(x, max_y, 1, tiles_to_change, new_tile)
}

fn generate_sea_water(world: &mut World, options: &TerrainOptions) {
    for x in 0..world.width {
        let mut y = world.height - options.sea_level;
        while world.tile_at(x, y) == TileType::Air {
            world.set_tile(x, y, TileType::Water);
            y += 1;
        }
    }
}
