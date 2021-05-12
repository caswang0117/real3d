use crate::serialization::*;
use crate::grid::*;
use serde_json;
use std::io::{Write, Read};

use std::fs::OpenOptions;
use std::fs::File;
use std::path::Path;

#[allow(unused_must_use)]
pub fn save<T: AsRef<Path>>(grid: &Grid, filename: T) {
    let sg = SerializableGrid::from_grid(grid);
    let s = serde_json::to_string(&sg).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename).unwrap();
    file.write(s.as_bytes());
    file.flush();
}

#[allow(unused_must_use)]
pub fn load<T: AsRef<Path>>(filename: T, default_origin:cgmath::Vector3<i32>) -> Grid {
    let file = File::open(filename);
    match file {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s);
            let sg: SerializableGrid = serde_json::from_str(s.as_str()).unwrap();
            sg.to_grid()
        }
        Err(_) => {
            return Grid::new(default_origin);
        }
    }
}