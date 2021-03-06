use crate::tiles::TileType;
use noise::{NoiseFn, Perlin, Seedable};

pub struct World {
    pub width: usize,
    pub height: usize,
    map: Vec<Vec<TileType>>,
    pub seed: u32,
}

impl World {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        Self {
            width,
            height,
            map: Self::generate_empty_map(width, height),
            seed,
        }
    }

    fn generate_empty_map(width: usize, height: usize) -> Vec<Vec<TileType>> {
        vec![vec![TileType::Air; width]; height]
    }

    pub fn tile_at(&self, x: usize, y: usize) -> TileType {
        *self.map
            .get(y)
            .expect("Can't get since Y coordinate is out of bounds!")
            .get(x)
            .expect("Can't get since X coordinate is out of bounds!")
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        let map_tile = self.map
            .get_mut(y)
            .expect("Can't set since Y coordinate is out of bounds!")
            .get_mut(x)
            .expect("Can't set since X coordinate is out of bounds!");
        *map_tile = tile;
    }

    pub fn fill_rectangle(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        tile: TileType,
    ) {
        let end_x = start_x + width;
        let end_y = start_y + height;
        if end_x > self.width || end_y > self.height {
            panic!(
                "Tried to fill tiles outside the map x: {} y: {}",
                end_x, end_y
            )
        }

        for y in start_y..end_y {
            for x in start_x..end_x {
                Self::set_tile(self, x, y, tile);
            }
        }
    }

    pub fn perlin(&self, x: usize, y: usize, scale: f64) -> f64 {
        // TODO: refactor so perlin is initialized once
        let perlin = Perlin::new().set_seed(self.seed);
        let value = perlin.get([x as f64 / scale, y as f64 / scale]);
        // convert <-1;1> to <0;1>
        value / 2.0 + 0.5
    }

    pub fn render_ascii_map(self) -> String {
        let mut map = String::from("");
        for y in self.map {
            let tiles: String = y.into_iter().map(|t| t.to_string()).collect();
            map.push_str(format!("|{}|\n", tiles).as_str());
        }
        map
    }
}
