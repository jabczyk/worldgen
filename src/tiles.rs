pub const AIR: u32 = 0;
pub const STONE: u32 = 1;

pub fn get_ascii_tile(tile: u32) -> &'static str {
    match tile {
        AIR => " ",
        STONE => "#",
        _ => "?",
    }
}
