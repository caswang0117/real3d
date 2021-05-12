use crate::assets::ModelRef;
use crate::geom::*;
use crate::geom::*;
use cgmath::Point3;
use rand::Rng;

pub type GridCoord = cgmath::Point3<i32>;
pub type TetrisBounds = cgmath::Point3<i32>; // top view, y is lowest point

use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeTuple;

pub const GRID_SCALE: f32 = 32.0;
pub const GRID_X_MAX: i32 = 8;
pub const GRID_Y_MAX: i32 = 16;
pub const GRID_Z_MAX: i32 = 8;


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Block {
    pub c: GridCoord,
    pub color: TetrisColor,
}


// impl Shape for Block {
//     fn translate(&mut self, v: Vec3) {
//         self.c += GridCoord::new(v.x as i32,v.y as i32,v.z as i32);
//     }
// }
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum TetrisColor {
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    Mix,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tetris {
    pub blocks: Vec<Block>,
    pub falling: bool,
}

impl Tetris {
    fn gen_random_tetris() -> Self {
        use TetrisColor::*;
        let mut rng = rand::thread_rng();
        let shape: usize = rng.gen_range(0..6);
        // println!("Shape {}", shape);
        match shape {
            0 => {
                // 2x2x2 cube
                Tetris {
                    blocks: vec![
                        Block {
                            c: GridCoord::new(5, 15, 5),
                            color: Red,
                        },
                        Block {
                            c: GridCoord::new(4, 15, 4),
                            color: Red,
                        },
                        Block {
                            c: GridCoord::new(5, 15, 4),
                            color: Red,
                        },
                        Block {
                            c: GridCoord::new(4, 15, 5),
                            color: Red,
                        },
                        Block {
                            c: GridCoord::new(5, 14, 5),
                            color: Red,
                        },
                        Block {
                            c: GridCoord::new(4, 14, 4),
                            color: Red,
                        },
                        Block {
                            c: GridCoord::new(5, 14, 4),
                            color: Red,
                        },
                        Block {
                            c: GridCoord::new(4, 14, 5),
                            color: Red,
                        },
                    ],
                    falling: true,
                }
            }
            1 => {
                Tetris {
                    // 4x1x1 line
                    blocks: vec![
                        Block {
                            c: GridCoord::new(4, 12, 4),
                            color: Green,
                        },
                        Block {
                            c: GridCoord::new(4, 13, 4),
                            color: Green,
                        },
                        Block {
                            c: GridCoord::new(4, 14, 4),
                            color: Green,
                        },
                        Block {
                            c: GridCoord::new(4, 15, 4),
                            color: Green,
                        },
                    ],
                    falling: true,
                }
            }
            2 => {
                Tetris {
                    // upside down T
                    blocks: vec![
                        Block {
                            c: GridCoord::new(5, 14, 4),
                            color: Blue,
                        },
                        Block {
                            c: GridCoord::new(5, 15, 4),
                            color: Blue,
                        },
                        Block {
                            c: GridCoord::new(4, 14, 4),
                            color: Blue,
                        },
                        Block {
                            c: GridCoord::new(6, 15, 4),
                            color: Blue,
                        },
                    ],
                    falling: true,
                }
            }
            3 => {
                // corner
                Tetris {
                    blocks: vec![
                        Block {
                            c: GridCoord::new(4, 14, 4),
                            color: Cyan,
                        },
                        Block {
                            c: GridCoord::new(4, 15, 4),
                            color: Cyan,
                        },
                        Block {
                            c: GridCoord::new(3, 14, 4),
                            color: Cyan,
                        },
                        Block {
                            c: GridCoord::new(4, 14, 5),
                            color: Cyan,
                        },
                    ],
                    falling: true,
                }
            }
            4 => {
                // Z
                Tetris {
                    blocks: vec![
                        Block {
                            c: GridCoord::new(4, 13, 4),
                            color: Magenta,
                        },
                        Block {
                            c: GridCoord::new(4, 14, 4),
                            color: Magenta,
                        },
                        Block {
                            c: GridCoord::new(5, 14, 4),
                            color: Magenta,
                        },
                        Block {
                            c: GridCoord::new(5, 15, 4),
                            color: Magenta,
                        },
                    ],
                    falling: true,
                }
            }
            5 => {
                Tetris {
                    // L
                    blocks: vec![
                        Block {
                            c: GridCoord::new(4, 13, 4),
                            color: Yellow,
                        },
                        Block {
                            c: GridCoord::new(5, 13, 4),
                            color: Yellow,
                        },
                        Block {
                            c: GridCoord::new(4, 14, 4),
                            color: Yellow,
                        },
                        Block {
                            c: GridCoord::new(4, 15, 4),
                            color: Yellow,
                        },
                    ],
                    falling: true,
                }
            }
            _ => Tetris {
                blocks: vec![],
                falling: false,
            }, // will not be reached
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize)]
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
    pub end: bool,
    pub grid: [GridBlock; (GRID_X_MAX * GRID_Y_MAX * GRID_Z_MAX) as usize],
}

impl Grid {
    pub fn new(origin: cgmath::Vector3<i32>) -> Self {
        Self {
            tetris: vec![Tetris::gen_random_tetris()],
            current: 0,
            origin,
            end: false,
            grid: [GridBlock::Vacant; (GRID_X_MAX * GRID_Y_MAX * GRID_Z_MAX) as usize],
        }
    }

    // pub fn index_occupied_by(&self, i:usize) -> Option<usize>{
    //     match self.grid[i]  {
    //         GridBlock::Vacant => None,
    //         GridBlock::Occupied(t) =>  Some(GridBlock::Occupied.0)
    //     }
    // }

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

    // check if any planes are full, return first full vec
    pub fn check_planes(&self) -> Vec<i32> {
        let mut planes = vec![];
        for i in 0..GRID_Y_MAX {
            let mut vacant = false;
            for g in self.get_plane(i) {
                if *g == GridBlock::Vacant {
                    vacant = true;
                }
            }
            if !vacant {
                planes.push(i);
            }
        }
        planes
    }

    // clear blocks on plane and lower blocks above
    // return new set of blocks for tetris piece
    pub fn split_piece(&mut self, i: usize, y: i32) -> Vec<Block> {
        let mut new_blocks = vec![];
        let tetris_vec = &mut self.tetris;

        for b in tetris_vec[i].blocks.iter_mut() {
            // if above cleared plane, lower by 1
            if b.c.y > y {
                self.grid[Self::coord_to_index(b.c)] = GridBlock::Vacant;
                b.c.y -= 1;
                self.grid[Self::coord_to_index(b.c)] = GridBlock::Occupied(i);
                new_blocks.push(b.clone());
            } else if b.c.y < y {
                self.grid[Self::coord_to_index(b.c)] = GridBlock::Vacant;
                new_blocks.push(b.clone());
                self.grid[Self::coord_to_index(b.c)] = GridBlock::Occupied(i);
            }
        }
        new_blocks
    }

    pub fn clear_plane(&mut self, y: i32) {
        debug_assert!(0 <= y && y < GRID_Y_MAX);
        // println!("clear function");
        let start_i = y * GRID_Z_MAX * GRID_X_MAX;
        let mut modified = vec![];
        let mut grid = self.grid;

        // go through every gridblock in this plane
        for (i, g) in grid
            [(y * GRID_Z_MAX * GRID_X_MAX) as usize..((y + 1) * GRID_Z_MAX * GRID_X_MAX) as usize]
            .iter_mut()
            .enumerate()
        {
            let coord = Grid::index_to_coord(start_i as usize + i);
            if let GridBlock::Occupied(tetris_i) = g {
                // don't split piece if already split
                if !modified.contains(tetris_i) {
                    let new_blocks = self.split_piece(*tetris_i, y);
                    self.tetris[*tetris_i].blocks = new_blocks;
                    modified.push(*tetris_i);
                }
            }
        }
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
    pub fn add_tetris(&mut self) {
        let tetris = Tetris::gen_random_tetris();
        // println!("add_tetris function");
        let i = self.tetris.len();
        for b in &tetris.blocks {
            // println!(" coord: {:?}", b.c);
            // println!("gridblock: {:?}", self.grid[Self::coord_to_index(b.c)]);
            if (!(self.grid[Self::coord_to_index(b.c)].is_vacant())
                && self.grid[Self::coord_to_index(b.c)] != GridBlock::Occupied(i))
            {
                self.end_game();
                return;
            }
            // debug_assert_eq!(self.grid[Self::coord_to_index(b.c)], GridBlock::Vacant);
            // println!("coord: {:?}", b.c);
            // println!("gridblock: {:?}", self.grid[Self::coord_to_index(b.c)]);
            self.grid[Self::coord_to_index(b.c)] = GridBlock::Occupied(i);
        }
        self.tetris.push(tetris);
        self.current = i;
    }

    // change all tetris colors and stop spawning new ones when game over
    pub fn end_game(&mut self) {
        println!("You died. GG.");
        for t in self.tetris.iter_mut() {
            for b in t.blocks.iter_mut() {
                b.color = TetrisColor::Mix;
            }
        }
        self.end = true;
    }

    pub fn tetris_at_xyz(&mut self, x: i32, y: i32, z: i32) -> Option<&mut Tetris> {
        let block = self.grid[Self::xyz_to_index(x, y, z)];
        match block {
            GridBlock::Occupied(i) => Some(&mut self.tetris[i]),
            GridBlock::Vacant => None,
        }
    }

    pub fn tetris_at_coord(&mut self, c: GridCoord) -> Option<&mut Tetris> {
        let (x, y, z) = (c.x, c.y, c.z);
        self.tetris_at_xyz(x, y, z)
    }

    // drop tetris by one grid spot
    pub fn lower_tetris(&mut self, i: usize) {
        let t = &mut self.tetris[i];
        // println!("lower_tetris function");

        // check if all spaces below are vacant or self and not floor
        for block in t.blocks.iter_mut() {
            let check = GridCoord::new(block.c.x, block.c.y - 1, block.c.z);
            // println!("check: {:?}", check);
            // println!(
            //     "if {:?}",
            //     check.y == -1
            //         || (!(self.grid[Self::coord_to_index(check)].is_vacant())
            //             && self.grid[Self::coord_to_index(check)] != GridBlock::Occupied(i))
            // );
            if check.y == -1
                || (!(self.grid[Self::coord_to_index(check)].is_vacant())
                    && self.grid[Self::coord_to_index(check)] != GridBlock::Occupied(i))
            {
                // println!("stop falling at {:?}", block.c);
                // println!("check y: {:?}", check.y);
                // println!(
                //     "check not vacant: {:?}",
                //     check.y == -1
                //         || (!(self.grid[Self::coord_to_index(check)].is_vacant())
                //             && self.grid[Self::coord_to_index(check)] != GridBlock::Occupied(i))
                // );
                // if check.y != -1 {
                //     println!(
                //         "occupied by: {:?}, current: {:?}",
                //         self.grid[Self::coord_to_index(check)],
                //         i
                //     );
                // }
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
                if check.y == -1
                    || (!(self.grid[Self::coord_to_index(check)].is_vacant())
                        && self.grid[Self::coord_to_index(check)] != GridBlock::Occupied(i))
                {
                    check_t.falling = false;
                } else {
                    // keep lowering until you can't
                    b.c.y -= 1;
                };
                // debug_assert!(Self::tetris_at_xyz(&self,block.c.x, block.c.y-1, block.c.z)==None);
                // self.grid[Self::coord_to_index(block.c)] = GridBlock::Vacant;
                // block.c.y -= 1;
                // block.falling = false;
            }
        }

        for b in &check_t.blocks {
            for a in t.blocks.iter_mut() {
                self.grid[Self::coord_to_index(a.c)] = GridBlock::Vacant;
                a.c = b.c; // change to lowest, final position
                self.grid[Self::coord_to_index(a.c)] = GridBlock::Occupied(i);
            }
        }
        t.falling = false;
    }

    // move tetris piece one grid spot in XZ plane
    // 0 left, 1 right, 2 up, 3 down
    pub fn move_xz(&mut self, i: usize, d: usize) {
        let t = &mut self.tetris[i];
        // println!("move_xz function");
        // don't move if will be out of grid
        if t.falling
        {
            match d {
                // Left
                0 => {
                    // println!("left");
                    // check move won't crash into any occupied spaces
                    for block in &t.blocks {
                        let check = GridCoord::new(block.c.x - 1, block.c.y, block.c.z);

                        if check.x == -1
                            || (!(self.grid[Self::coord_to_index(check)].is_vacant())
                                && self.grid[Self::coord_to_index(check)] != GridBlock::Occupied(i))
                        {
                            return;
                        };
                    }


                    // move each block in tetris left 1
                    for block in t.blocks.iter_mut() {
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Vacant;
                        block.c.x -= 1;
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Occupied(i);
                    }
                }
                // Right
                1 => {
                    // check move won't crash into any occupied spaces
                    for block in &t.blocks {
                        let check = GridCoord::new(block.c.x + 1, block.c.y, block.c.z);
                        if check.x == 8
                            || (!(self.grid[Self::coord_to_index(check)].is_vacant())
                                && self.grid[Self::coord_to_index(check)] != GridBlock::Occupied(i))
                        {
                            return;
                        };
                    }


                    // move each block in tetris left 1
                    for block in t.blocks.iter_mut() {
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Vacant;
                        block.c.x += 1;
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Occupied(i);
                    }
                }
                // Up
                2 => {
                    // check move won't crash into any occupied spaces
                    for block in &t.blocks {
                        let check = GridCoord::new(block.c.x, block.c.y, block.c.z + 1);
                        if check.z == 8
                            || (!(self.grid[Self::coord_to_index(check)].is_vacant())
                                && self.grid[Self::coord_to_index(check)] != GridBlock::Occupied(i))
                        {
                            return;
                        };
                    }

                    // move each block in tetris left 1
                    for block in t.blocks.iter_mut() {
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Vacant;
                        block.c.z += 1;
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Occupied(i);
                    }
                }
                // Down
                3 => {
                    // check move won't crash into any occupied spaces
                    for block in &t.blocks {
                        let check = GridCoord::new(block.c.x, block.c.y, block.c.z - 1);
                        if check.z == -1
                            || (!(self.grid[Self::coord_to_index(check)].is_vacant())
                                && self.grid[Self::coord_to_index(check)] != GridBlock::Occupied(i))
                        {
                            return;
                        };
                    }

                    // move each block in tetris left 1
                    for block in t.blocks.iter_mut() {
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Vacant;
                        block.c.z -= 1;
                        self.grid[Self::coord_to_index(block.c)] = GridBlock::Occupied(i);
                    }
                }
                _ => (),
            }
        }
    }
}
