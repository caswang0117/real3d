use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeTuple;
use serde::de::DeserializeOwned;
use crate::grid::*;

pub type SerializablePos3<T> = [T; 3];

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableBlock {
    pub color: TetrisColor,
    pub c: SerializablePos3<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableTetris {
    pub blocks: Vec<SerializableBlock>,
    pub falling: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableGrid {
    pub tetris: Vec<SerializableTetris>,
    pub current: usize,
    pub origin: SerializablePos3<i32>,
    // end is not necessary because ended grid will be serialized as empty on disk
}

impl SerializableBlock {
    pub fn from_block(b: &Block) -> Self {
        Self {
            color: b.color,
            c: [b.c.x, b.c.y, b.c.z],
        }
    }

    pub fn to_block(&self) -> Block {
        Block {
            color: self.color,
            c: GridCoord::new(self.c[0], self.c[1], self.c[2]),
        }
    }
}

impl SerializableTetris {
    pub fn from_tetris(t: &Tetris) -> Self {
        Self {
            falling: t.falling,
            blocks: t.blocks.iter().map(|b| SerializableBlock::from_block(b)).collect(),
        }
    }

    pub fn to_tetris(&self) -> Tetris {
        Tetris {
            falling: self.falling,
            blocks: self.blocks.iter().map(|b| b.to_block()).collect(),
        }
    }
}

impl SerializableGrid {
    pub fn from_grid(g: &Grid) -> Self {
        Self{
            current:g.current,
            origin:[g.origin.x,g.origin.y,g.origin.z],
            tetris:g.tetris.iter().map(|t| SerializableTetris::from_tetris(t)).collect()
        }
    }

    pub fn to_grid(&self)->Grid{
        let mut grid=[GridBlock::Vacant; (GRID_X_MAX * GRID_Y_MAX * GRID_Z_MAX) as usize];
        let mut tetris:Vec<Tetris>=vec![];
        for (i,st) in self.tetris.iter().enumerate(){
            let t=st.to_tetris();
            for b in t.blocks.iter(){
                grid[Grid::coord_to_index(b.c)]=GridBlock::Occupied(i);
            }
            tetris.push(t);
        }

        Grid{
            tetris,
            current: self.current,
            origin: cgmath::Vector3::<i32>::new(self.origin[0],self.origin[1],self.origin[2]),
            end: false,
            grid
        }
    }
}


