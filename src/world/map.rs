#[derive(Debug)]
pub enum TileType {
    Wall,
    Ground,
    Air,
}

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

        for _ in 0..vec.capacity() {
            vec.push(Tile { tile_type: TileType::Ground } );
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
