use crate::tiles::get_ascii_tile;

pub struct World {
    pub map: Vec<Vec<u32>>,
}

impl World {
    pub fn set_tile(&mut self, x: usize, y: usize, block: u32) {
        self.map[y][x] = block
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

pub fn generate_map(size_x: usize, size_y: usize) -> Vec<Vec<u32>> {
    vec![vec![0u32; size_x]; size_y]
}
