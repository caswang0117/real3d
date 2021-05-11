use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeTuple;
use serde::de::DeserializeOwned;
use crate::grid::*;

pub type SerializablePos3<T> = [T; 3];

#[derive(Serialize, Deserialize,Debug)]
pub struct SerializableBlock {
    color: TetrisColor,
    c: SerializablePos3<i32>,
}

impl SerializableBlock{
    fn from_block(b:&Block)->Self{
        Self{
            color:b.color,
            c:[b.c.x,b.c.y,b.c.z]
        }
    }

    fn to_block(&self)->Block{
        Block{
            color:self.color,
            c:GridCoord::new(self.c[0],self.c[1],self.c[2])
        }
    }
}


#[derive(Serialize, Deserialize,Debug)]
pub struct SerializableTetris {
    blocks: Vec<SerializableBlock>,
    falling: bool,
}

impl SerializableTetris{
    fn from_tetris(t:&Tetris)->Self{
        Self{
            falling:t.falling,
            blocks:t.blocks.iter().map(|b| SerializableBlock::from_block(b)).collect()
        }
    }

    fn to_tetris(&self)->Tetris{
        Tetris{
            falling:self.falling,
            blocks:self.blocks.iter().map(|b| b.to_block()).collect()
        }
    }
}



