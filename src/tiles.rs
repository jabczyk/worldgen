use std::{fmt, convert};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Air,
    Stone,
    Dirt,
    Sand,
    Water
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TileType::Air => write!(f, " "),
            TileType::Stone => write!(f, "#"),
            TileType::Dirt => write!(f, "*"),
            TileType::Sand => write!(f, "%"),
            TileType::Water => write!(f, "_"),
        }
    }
}
impl convert::Into<u32> for TileType {
    fn into(self) -> u32 {
        match self {
            TileType::Air => 0u32,
            TileType::Stone => 1u32,
            TileType::Dirt => 2u32,
            TileType::Sand => 3u32,
            TileType::Water => 4u32,
        }
    }
}
