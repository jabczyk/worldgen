use crate::tiles::get_ascii_tile;

pub struct World {
    pub width: usize,
    pub height: usize,
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

    pub fn fill_rectangle(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        tile: u32,
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
                World::set_tile(self, x, y, tile);
            }
        }
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
