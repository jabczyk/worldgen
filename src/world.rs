use crate::tiles::get_ascii_tile;

pub struct World {
    width: usize,
    height: usize,
    map: Vec<Vec<u32>>,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        World {
            width,
            height,
            map: World::generate_empty_map(width, height),
        }
    }

    fn generate_empty_map(width: usize, height: usize) -> Vec<Vec<u32>> {
        vec![vec![0u32; width]; height]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: u32) {
        if x >= self.width || y >= self.height {
            panic!("Tried to set tile outside the map x: {} y: {}", x, y)
        }
        self.map[y][x] = tile
    }

    pub fn render_ascii_map(self) -> String {
        let mut map = String::from("");
        for y in self.map {
            let tiles: String = y.into_iter().map(get_ascii_tile).collect();
            map.push_str(format!("|{}|\n", tiles).as_str());
        }
        map
    }
}
