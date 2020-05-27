pub const AIR: u32 = 0;
pub const STONE: u32 = 1;
pub const DIRT: u32 = 2;
pub const SAND: u32 = 3;
pub const WATER: u32 = 4;

pub fn get_ascii_tile(tile: u32) -> &'static str {
    match tile {
        AIR => " ",
        STONE => "#",
        DIRT => "*",
        SAND => "%",
        WATER => "_",
        _ => "?",
    }
}
