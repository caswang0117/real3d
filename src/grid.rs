use cgmath;
use rand::Rng;

pub type GridCoord = cgmath::Point3<i32>;

pub const GRID_SCALE: f32 = 32.0;
pub const GRID_X_MAX: i32 = 8;
pub const GRID_Y_MAX: i32 = 16;
pub const GRID_Z_MAX: i32 = 8;


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Block {
    pub c: GridCoord,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tetris {
    pub blocks: Vec<Block>,
    pub falling: bool,
}

impl Tetris {
    fn gen_random_tetris() -> Self {
        let mut rng = rand::thread_rng();
        let shape: usize = rng.gen_range(0..5);
        match shape {
            0 => {    // 2x2x2 cube
                Tetris {
                    blocks: vec![Block { c: GridCoord::new(5, 15, 5) },
                                 Block { c: GridCoord::new(6, 15, 6) },
                                 Block { c: GridCoord::new(4, 15, 6) },
                                 Block { c: GridCoord::new(6, 15, 5) },
                                 Block { c: GridCoord::new(5, 14, 5) },
                                 Block { c: GridCoord::new(6, 14, 6) },
                                 Block { c: GridCoord::new(4, 14, 6) },
                                 Block { c: GridCoord::new(6, 14, 5) }
                    ],
                    falling: true,
                }
            }
            1 => {
                Tetris {    // 4x1x1 line
                    blocks: vec![Block { c: GridCoord::new(5, 13, 5) },
                                 Block { c: GridCoord::new(5, 13, 5) },
                                 Block { c: GridCoord::new(5, 14, 5) },
                                 Block { c: GridCoord::new(5, 15, 5) }],
                    falling: true,
                }
            }
            2 => {
                Tetris { // upside down T
                    blocks: vec![Block { c: GridCoord::new(5, 14, 5) },
                                 Block { c: GridCoord::new(5, 15, 5) },
                                 Block { c: GridCoord::new(4, 14, 5) },
                                 Block { c: GridCoord::new(6, 15, 5) }],
                    falling: true,
                }
            }
            3 => {
                // corner
                Tetris {
                    blocks: vec![Block { c: GridCoord::new(4, 14, 4) },
                                 Block { c: GridCoord::new(4, 15, 4) },
                                 Block { c: GridCoord::new(3, 14, 4) },
                                 Block { c: GridCoord::new(4, 14, 5) }],
                    falling: true,
                }
            }
            4 => {
                // Z
                Tetris {
                    blocks: vec![Block { c: GridCoord::new(3, 13, 3) },
                                 Block { c: GridCoord::new(3, 14, 3) },
                                 Block { c: GridCoord::new(4, 14, 3) },
                                 Block { c: GridCoord::new(4, 15, 3) }],
                    falling: true,
                }
            }
            5 => {
                Tetris { // L
                    blocks: vec![Block { c: GridCoord::new(3, 13, 3) },
                                 Block { c: GridCoord::new(4, 13, 3) },
                                 Block { c: GridCoord::new(3, 14, 3) },
                                 Block { c: GridCoord::new(3, 15, 3) }],
                    falling: true,
                }
            }
            _ => { Tetris { blocks: vec![], falling: false } } // will not be reached
        }
    }
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GridBlock {
    Vacant,
    Occupied(usize),
}

pub struct Grid {
    pub tetris: Vec<Tetris>,
    grid: [GridBlock; (GRID_X_MAX * GRID_Y_MAX * GRID_Z_MAX) as usize],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            tetris: vec![],
            grid: [GridBlock::Vacant; (GRID_X_MAX * GRID_Y_MAX * GRID_Z_MAX) as usize],
        }
    }

    pub fn xyz_to_index(x: i32, y: i32, z: i32) -> usize {
        debug_assert!(0 <= x && x < GRID_X_MAX);
        debug_assert!(0 <= y && y < GRID_Y_MAX);
        debug_assert!(0 <= z && z < GRID_Z_MAX);
        (z + x * GRID_Z_MAX + y * GRID_Z_MAX * GRID_X_MAX) as usize
    }

    pub fn index_to_xyz(i: usize) -> (i32, i32, i32) {
        let i = i as i32;
        debug_assert!(i < GRID_X_MAX * GRID_Y_MAX * GRID_Z_MAX);

        let z = i % GRID_Z_MAX;
        let tmp = i / GRID_Z_MAX;

        let x = tmp % GRID_X_MAX;
        let y = tmp / GRID_X_MAX;

        debug_assert!(0 <= x && x < GRID_X_MAX);
        debug_assert!(0 <= y && y < GRID_Y_MAX);
        debug_assert!(0 <= z && z < GRID_Z_MAX);

        (x, y, z)
    }

    pub fn get_plane(&self, y: i32) -> &[GridBlock] {
        debug_assert!(0 <= y && y < GRID_Y_MAX);
        &self.grid[(y * GRID_Z_MAX * GRID_X_MAX) as usize..((y + 1) * GRID_Z_MAX * GRID_X_MAX) as usize]
    }
    pub fn coord_to_index(c: GridCoord) -> usize {
        Self::xyz_to_index(c.x, c.y, c.z)
    }

    pub fn index_to_coord(i: usize) -> GridCoord {
        let (x, y, z) = Self::index_to_xyz(i);
        GridCoord::new(x, y, z)
    }

    pub fn add_tetris(&mut self, tetris: Tetris) {
        let i = self.tetris.len();
        for b in &tetris.blocks {
            debug_assert_eq!(self.grid[Self::coord_to_index(b.c)], GridBlock::Vacant);
            self.grid[Self::coord_to_index(b.c)] = GridBlock::Occupied(i)
        }
        self.tetris.push(tetris);
    }

    pub fn tetris_at_xyz(&self, x: i32, y: i32, z: i32) -> Option<&Tetris> {
        let block = self.grid[Self::xyz_to_index(x, y, z)];
        match block {
            GridBlock::Occupied(i) => { Some(&self.tetris[i]) }
            GridBlock::Vacant => { None }
        }
    }

    pub fn tetris_at_coord(&self, c: GridCoord) -> Option<&Tetris> {
        let (x, y, z) = (c.x, c.y, c.z);
        self.tetris_at_xyz(x, y, z)
    }

    pub fn lower_tetris(&mut self, i: usize) { // drop tetris by one
        let t = &mut self.tetris[i];
        // debug_assert!(Self::tetris_at_xyz(&self,block.c.x, block.c.y-1, block.c.z)==None);
    }

    pub fn probe_lowest(&self, x: i32, y: i32, z: i32, i: usize) {}

    pub fn drop_tetris(&mut self, i: usize) { // drop to lowest possible position
        let t = &mut self.tetris[i];
        for block in t.blocks.iter_mut() {
            // check if block below is empty or not self
            // debug_assert!(Self::tetris_at_xyz(&self,block.c.x, block.c.y-1, block.c.z)==None);
            self.grid[Self::coord_to_index(block.c)] = GridBlock::Vacant;
            block.c.y -= 1;
            // block.falling = false;
        }

    }
}

