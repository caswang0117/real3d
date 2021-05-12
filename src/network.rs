use std;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::FromStr;
use crate::serialization::SerializableGrid;
use crate::grid::Grid;

const BUFSIZE: usize = 4096;

pub struct Server {
    pub id: i32,
    sock: Option<TcpStream>,
    waiting: bool,
    pub connected: bool,
    pub has_other: bool,
}

#[allow(dead_code)]
impl Server {
    pub fn new() -> Server {
        Server {
            id: -1,
            connected: false,
            waiting: false,
            has_other: false,
            sock: None,
        }
    }

    pub fn connect(&mut self, addr: &str) {
        let addr = std::net::SocketAddr::from_str(addr).unwrap();
        let mut stream;
        match TcpStream::connect(addr) {
            Ok(s) => stream = s,
            Err(e) => {
                println!("Cannot connect to server ({}). Using offline mode", e);
                return;
            }
        }
        let mut buf: [u8; BUFSIZE] = [0; BUFSIZE]; // well memory is cheap
        stream.read(&mut buf).unwrap();
        stream.set_nonblocking(true).unwrap();
        let s = std::str::from_utf8(&buf).unwrap();
        let term = s.find("\n").unwrap();
        let id = s[..term].parse::<i32>().unwrap();
        if id == -1 {
            println!("Server capacity reached. Using offline mode. Please try again later");
            return;
        }
        println!("Server connection established");
        self.id = id;
        self.connected = true;
        self.sock = Some(stream);
    }

    fn disconnect(&mut self) {
        let mut sock = self.sock.as_ref().unwrap();
        sock.write("{\"op\":\"disconnect\"}\n".as_bytes()).unwrap();
        sock.flush().unwrap();
        sock.shutdown(Shutdown::Both).unwrap();
    }

    fn update(&mut self, grid: &SerializableGrid) -> Result<Vec<SerializableGrid>, Box<dyn std::error::Error>> {
        if !self.connected || self.waiting {
            return Ok(Vec::<SerializableGrid>::new()); // empty vec
        }
        let mut sock = self.sock.as_ref().unwrap();
        let obj = serde_json::json!({
            "op":"update",
            "data":grid
        });
        let j = serde_json::to_string(&obj).unwrap() + "\n";
        sock.write(j.as_bytes())?;
        sock.flush().unwrap();
        let mut buf: [u8; BUFSIZE] = [0; BUFSIZE];
        sock.read(&mut buf)?;
        let s = std::str::from_utf8(&mut buf)?;
        if let Some(term) = s.find("\n") {
            let v: Vec<SerializableGrid> = serde_json::from_str(&s[..term])?;
            // println!("Instance {} Recved from server: {}", self.id, s);
            if v.len()==0{
                self.has_other=false;
            } else {
                self.has_other=true;
            }
            Ok(v)
        } else {
            // println!("unexpected error. something went wrong");
            Ok(Vec::<SerializableGrid>::new())

            // this is not ok but i can't get rust to throw something sensible
        }
    }

    pub fn update_grid(&mut self, grid: &Grid) -> Vec<SerializableGrid> {
        let sg = SerializableGrid::from_grid(grid);
        let response = self.update(&sg);
        match response {
            Ok(v) => {
                self.waiting = false;
                return v;
            }
            _ => {
                self.waiting = true;
                return vec![];
            }
        }
    }
}

impl Drop for Server {
    // destructor
    fn drop(&mut self) {
        if self.connected {
            self.disconnect();
        }
    }
}