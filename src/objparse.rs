use std::io::prelude::*;
use std::fs::{File};

// NOTE(Lito): We are ignoring Normals and Objects for now.
pub struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Face {
    pub verts: Vec<u32>
}

pub struct Model {
    // pub faces: Vec<Face>,
    // pub verts: Vec<Vertex>,
    // normals
    // objects
}

fn parse_obj_face(line) -> Face {
}

fn parse_obj_vert(line) -> Vertex {
}

pub fn parse(file: String) -> Model {
    let mut verts : Vec<Vertex> = Vec::new();
    let mut faces : Vec<Face>   = Vec::new();
    for line in file.lines() {
        let words = line.split_whitespace().collect(); {
            let kind : Option<&str> = words.first();
            match kind {
                Some("v") => verts.push(parse_obj_vert(line)),
                Some("f") => faces.push(parse_obj_face(line)),
                _   => continue
            }
        }
    }
    Model { }
}

pub fn read(filepath: &str) -> String {
    let mut f = File::open(filepath).unwrap();

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}
