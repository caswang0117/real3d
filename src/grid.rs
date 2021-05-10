use cgmath::Point3;
use rand::Rng;
use crate::geom::*;

pub type GridCoord = cgmath::Point3<i32>;
pub type TetrisBounds = cgmath::Point3<i32>; // top view, y is lowest point

pub const GRID_SCALE: f32 = 32.0;
pub const GRID_X_MAX: i32 = 8;
pub const GRID_Y_MAX: i32 = 16;
pub const GRID_Z_MAX: i32 = 8;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Block {
    pub c: GridCoord,
}

// impl Shape for Block {
//     fn translate(&mut self, v: Vec3) {
//         self.c += GridCoord::new(v.x as i32,v.y as i32,v.z as i32);
//     }
// }

#[derive(Clone, PartialEq, Debug)]
pub struct Tetris {
    pub blocks: Vec<Block>,
    pub bounds: TetrisBounds,
    pub falling: bool,
}

impl Tetris {
    fn gen_random_tetris() -> Self {
        let mut rng = rand::thread_rng();
        let shape: usize = rng.gen_range(0..6);
        println!("Shape {}",shape);
        match shape {
            0 => {
                // 2x2x2 cube
                Tetris {
                    blocks: vec![
                        Block {
                            c: GridCoord::new(5, 15, 5),
                        },
                        Block {
                            c: GridCoord::new(6, 15, 6),
                        },
                        Block {
                            c: GridCoord::new(4, 15, 6),
                        },
                        Block {
                            c: GridCoord::new(6, 15, 5),
                        },
                        Block {
                            c: GridCoord::new(5, 14, 5),
                        },
                        Block {
                            c: GridCoord::new(6, 14, 6),
                        },
                        Block {
                            c: GridCoord::new(4, 14, 6),
                        },
                        Block {
                            c: GridCoord::new(6, 14, 5),
                        },
                    ],
                    bounds: TetrisBounds::new(4, 14, 6),
                    falling: true,
                }
            }
            1 => {
                Tetris {
                    // 4x1x1 line
                    blocks: vec![
                        Block {
                            c: GridCoord::new(5, 13, 5),
                        },
                        Block {
                            c: GridCoord::new(5, 13, 5),
                        },
                        Block {
                            c: GridCoord::new(5, 14, 5),
                        },
                        Block {
                            c: GridCoord::new(5, 15, 5),
                        },
                    ],
                    bounds: TetrisBounds::new(5, 13, 5),
                    falling: true,
                }
            }
            2 => {
                Tetris {
                    // upside down T
                    blocks: vec![
                        Block {
                            c: GridCoord::new(5, 14, 5),
                        },
                        Block {
                            c: GridCoord::new(5, 15, 5),
                        },
                        Block {
                            c: GridCoord::new(4, 14, 5),
                        },
                        Block {
                            c: GridCoord::new(6, 15, 5),
                        },
                    ],
                    bounds: TetrisBounds::new(4, 14, 5),
                    falling: true,
                }
            }
            3 => {
                // corner
                Tetris {
                    blocks: vec![
                        Block {
                            c: GridCoord::new(4, 14, 4),
                        },
                        Block {
                            c: GridCoord::new(4, 15, 4),
                        },
                        Block {
                            c: GridCoord::new(3, 14, 4),
                        },
                        Block {
                            c: GridCoord::new(4, 14, 5),
                        },
                    ],
                    bounds: TetrisBounds::new(3, 14, 5),
                    falling: true,
                }
            }
            4 => {
                // Z
                Tetris {
                    blocks: vec![
                        Block {
                            c: GridCoord::new(3, 13, 3),
                        },
                        Block {
                            c: GridCoord::new(3, 14, 3),
                        },
                        Block {
                            c: GridCoord::new(4, 14, 3),
                        },
                        Block {
                            c: GridCoord::new(4, 15, 3),
                        },
                    ],
                    bounds: TetrisBounds::new(3, 13, 3),
                    falling: true,
                }
            }
            5 => {
                Tetris {
                    // L
                    blocks: vec![
                        Block {
                            c: GridCoord::new(3, 13, 3),
                        },
                        Block {
                            c: GridCoord::new(4, 13, 3),
                        },
                        Block {
                            c: GridCoord::new(3, 14, 3),
                        },
                        Block {
                            c: GridCoord::new(3, 15, 3),
                        },
                    ],
                    bounds: TetrisBounds::new(3, 13, 3),
                    falling: true,
                }
            }
            _ => Tetris {
                blocks: vec![],
                bounds: TetrisBounds::new(0, 0, 0),
                falling: false,
            }, // will not be reached
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GridBlock {
    Vacant,
    Occupied(usize),
}

impl GridBlock {
    pub fn is_vacant(&self) -> bool {
        match *self {
            GridBlock::Vacant => true,
            _ => false,
        }
    }
}

pub struct Grid {
    pub tetris: Vec<Tetris>,
    pub current: usize,
    pub origin: cgmath::Vector3<i32>,
    grid: [GridBlock; (GRID_X_MAX * GRID_Y_MAX * GRID_Z_MAX) as usize],
}

impl Grid {
    pub fn new(origin: cgmath::Vector3<i32>) -> Self {
        Self {
            tetris: vec![
                Tetris::gen_random_tetris()
            ],
            current: 0,
            origin,
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
        &self.grid
            [(y * GRID_Z_MAX * GRID_X_MAX) as usize..((y + 1) * GRID_Z_MAX * GRID_X_MAX) as usize]
    }
    // takes a grid coordinate, returns index in grid struct
    pub fn coord_to_index(c: GridCoord) -> usize {
        Self::xyz_to_index(c.x, c.y, c.z)
    }

    // takes index in grid struct, returns grid coordinate
    pub fn index_to_coord(i: usize) -> GridCoord {
        let (x, y, z) = Self::index_to_xyz(i);
        GridCoord::new(x, y, z)
    }

    // spawn new tetris piece
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
            GridBlock::Occupied(i) => Some(&self.tetris[i]),
            GridBlock::Vacant => None,
        }
    }

    pub fn tetris_at_coord(&self, c: GridCoord) -> Option<&Tetris> {
        let (x, y, z) = (c.x, c.y, c.z);
        self.tetris_at_xyz(x, y, z)
    }

    // drop tetris by one grid spot
    pub fn lower_tetris(&mut self, i: usize) {
        let t = &mut self.tetris[i];

        // check if all spaces below are vacant or self and not floor
        for block in t.blocks.iter_mut() {
            let check = GridCoord::new(block.c.x, block.c.y - 1, block.c.z);

            if !(self.grid[Self::coord_to_index(check)]).is_vacant()
                || !(self.grid[Self::coord_to_index(check)] == GridBlock::Occupied(i))
                || check.y == 0
            {
                t.falling = false;
                return;
            };
        }

        // lower tetris by 1
        for block in t.blocks.iter_mut() {
            self.grid[Self::coord_to_index(block.c)] = GridBlock::Vacant;
            block.c.y -= 1;
            self.grid[Self::coord_to_index(block.c)] = GridBlock::Occupied(i);
        }
    }

    pub fn probe_lowest(&self, x: i32, y: i32, z: i32, i: usize) {}

    // drop to lowest possible position
    pub fn drop_tetris(&mut self, i: usize) {
        let t = &mut self.tetris[i];

        let mut check_t = t.clone();

        while check_t.falling {
            for b in check_t.blocks.iter_mut() {
                let check = GridCoord::new(b.c.x, b.c.y - 1, b.c.z);
                // check that space below is vacant and not self and not below grid
                if check.y == 0 {
                    check_t.falling = false;
                } else if self.grid[Self::coord_to_index(check)].is_vacant()
                    || self.grid[Self::coord_to_index(check)] == GridBlock::Occupied(i)
                    || check.y > 0
                {
                    // keep lowering until you can't
                    b.c.y -= 1;
                };
                // debug_assert!(Self::tetris_at_xyz(&self,block.c.x, block.c.y-1, block.c.z)==None);
                // self.grid[Self::coord_to_index(block.c)] = GridBlock::Vacant;
                // block.c.y -= 1;
                // block.falling = false;
            }
        }
        t.falling = false;
        for b in &check_t.blocks {
            for a in t.blocks.iter_mut() {
                self.grid[Self::coord_to_index(a.c)] = GridBlock::Vacant;
                a.c = b.c; // change to lowest, final position
                self.grid[Self::coord_to_index(a.c)] = GridBlock::Occupied(i);
            }
        }
    }

    // move tetris piece one grid spot in XZ plane
    pub fn move_xz(&mut self, i: usize, d: usize) {
        let t = &mut self.tetris[i];

        // don't move if will be out of grid
        if (0 <= t.bounds.x && t.bounds.x < GRID_X_MAX)
            && (0 <= t.bounds.z && t.bounds.z < GRID_Z_MAX)
        {
            match d {
                // Left
                0 => {
                    // check move won't crash into any occupied spaces
                    for block in &t.blocks {
                        let check = GridCoord::new(block.c.x - 1, block.c.y, block.c.z);
                        if !(self.grid[Self::coord_to_index(check)]).is_vacant() {
                            return;
                        };
                    }

                    let mut new_x = t.bounds.x;

                    // move each block in tetris left 1
                    for block in t.blocks.iter_mut() {
                        block.c.x -= 1;
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Occupied(i);
                        // find new left bound
                        if block.c.x < new_x {
                            new_x = block.c.x;
                        }
                    }
                    // update bounds
                    t.bounds.x = new_x;
                }
                // Right
                1 => {
                    // check move won't crash into any occupied spaces
                    for block in &t.blocks {
                        let check = GridCoord::new(block.c.x + 1, block.c.y, block.c.z);
                        if !(self.grid[Self::coord_to_index(check)]).is_vacant() {
                            return;
                        };
                    }

                    let mut new_x = t.bounds.x;

                    // move each block in tetris left 1
                    for block in t.blocks.iter_mut() {
                        block.c.x += 1;
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Occupied(i);
                        // find new right bound
                        if block.c.x > new_x {
                            new_x = block.c.x;
                        }
                    }
                    // update bounds
                    t.bounds.x = new_x;
                }
                // Up
                2 => {
                    // check move won't crash into any occupied spaces
                    for block in &t.blocks {
                        let check = GridCoord::new(block.c.x, block.c.y, block.c.z + 1);
                        if !(self.grid[Self::coord_to_index(check)]).is_vacant() {
                            return;
                        };
                    }

                    let mut new_z = t.bounds.z;

                    // move each block in tetris left 1
                    for block in t.blocks.iter_mut() {
                        block.c.z += 1;
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Occupied(i);
                        // find new top bound
                        if block.c.z > new_z {
                            new_z = block.c.z;
                        }
                    }
                    // update bounds
                    t.bounds.z = new_z;
                }
                // Down
                3 => {
                    // check move won't crash into any occupied spaces
                    for block in &t.blocks {
                        let check = GridCoord::new(block.c.x, block.c.y, block.c.z - 1);
                        if !(self.grid[Self::coord_to_index(check)]).is_vacant() {
                            return;
                        };
                    }

                    let mut new_z = t.bounds.z;

                    // move each block in tetris left 1
                    for block in t.blocks.iter_mut() {
                        block.c.z -= 1;
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Occupied(i);
                        // find new bottom bound
                        if block.c.z < new_z {
                            new_z = block.c.z;
                        }
                    }
                    // update bounds
                    t.bounds.z = new_z;
                }
                _ => (),
            }
        }
    }
}
