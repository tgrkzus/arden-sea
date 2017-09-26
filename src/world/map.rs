extern crate noise;
use self::noise::{NoiseModule, Perlin, Seedable};

#[derive(Debug)]
pub enum TileType {
    Wall,
    Ground,
    Air, }

#[derive(Debug)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let mut vec: Vec<Tile> = Vec::with_capacity(width * height * depth);

        let perlin = Perlin::new();
        perlin.set_seed(133213);
        for i in 0..vec.capacity() {
            let x = i / (height * depth);
            let y = (i - x * height * depth) / depth;
            let z = i - x * height * depth - y * depth;

            let modifier = 1000.0;
            let n = perlin.get([
                               x as f32 / width as f32 * modifier, 
                               y as f32 / height as f32 * modifier, 
                               z as f32 / depth as f32 * modifier]);

            //println!("{} {} {}: {}", x, y, z, n);
            let tile: Tile;
            if (n > 0.8) {
                tile = Tile { tile_type: TileType::Wall };
            }
            else {
                tile = Tile { tile_type: TileType::Ground };
            }
                vec.push(tile);
        }

        return Self { 
            width: width,
            height: height,
            depth: depth,
            tiles: vec,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Option<&Tile> {
        //println!("{}" , x + self.width * (y + self.height * z));
        // x + w * (y + HEIGHT * z)
        return self.tiles.get(x + self.width * (y + self.height * z));
    }

    pub fn set_tile(&mut self, tile: Tile, x: usize, y: usize, z: usize) {
        self.tiles[x + self.width * (y + self.height * z)] = tile;
    }
}
