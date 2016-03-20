use std::io::prelude::*;
use std::fs::{File};

// NOTE(Lito): We are ignoring Normals and Objects for now.
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
pub struct Face {
    pub verts: Vec<usize>
}

#[derive(Debug)]
pub struct Model {
    pub faces: Vec<Face>,
    pub verts: Vec<Vertex>,
    // normals
    // objects
}

fn parse_obj_face(line: &str) -> Face {
    // f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3
    let verts : Vec<usize> =
        line.split_whitespace().skip(1) // get down to vertex groups
        .map(|vgroup| {
            let vertex : &str = vgroup.split("/").collect::<Vec<&str>>().first().unwrap();
            match vertex.parse::<usize>() {
                Ok(v)  => v,
                Err(e) => panic!("{}, {}", e, vertex),
            }
        }).collect();
    Face { verts: verts }
}

fn parse_obj_vert(line: &str) -> Vertex {
    // v x y z
    let verts : Vec<f64> = line.split_whitespace()
                               .filter_map(|c| c.parse::<f64>().ok() )
                               .collect();
    Vertex {
        x: verts[0],
        y: verts[1],
        z: verts[2],
    }
}

pub fn parse(file: String) -> Model {
    let mut verts : Vec<Vertex> = Vec::new();
    let mut faces : Vec<Face>   = Vec::new();
    for line in file.lines() {
        let leading_str = line.split(" ").next();
        match leading_str {
            Some("v") => verts.push(parse_obj_vert(line)),
            Some("f") => faces.push(parse_obj_face(line)),
            _   => continue
        }
    }

    Model {
        verts: verts,
        faces: faces,
    }
}

pub fn read(path: &str) -> String {
    let mut f = File::open(path).unwrap();

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

pub fn load(path: &str) -> Model {
    let objfile : String = read(path);
    parse(objfile)
}
