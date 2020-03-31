use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Face {
    U,
    R,
    F,
    D,
    L,
    B,
}

#[derive(Debug)]
pub struct RawFace {
    colors: [[char; 3]; 3],
}

#[derive(Debug)]
pub struct RawCube {
    faces: HashMap<Face, RawFace>,
}

impl RawCube {
    pub fn face_colors(&self) -> HashMap<Face, char> {
        self.faces.iter()
            .map(|(&k, v)| (k, v.colors[1][1]))
            .collect()
    }

    pub fn color_to_faces(&self) -> HashMap<char, Face> {
        self.faces.iter()
            .map(|(&k, v)| (v.colors[1][1], k))
            .collect()
    }

    fn fetch_point(&self, pt: &(Face, usize, usize)) -> Option<char> {
        match self.faces.get(&pt.0) {
            Some(face) => Some(face.colors[pt.1][pt.2]),
            None => None
        }
    }

    pub fn facelet(&self) {
        //let corners: [[(Face, usize, usize); 3]; 8] = [
        let corners = [
            [(Face::U, 2, 0), (Face::L, 0, 2), (Face::F, 0, 0)],
            [(Face::U, 2, 2), (Face::F, 0, 2), (Face::R, 0, 0)],
            [(Face::U, 0, 2), (Face::R, 0, 2), (Face::B, 0, 0)],
            [(Face::U, 0, 0), (Face::B, 0, 2), (Face::L, 0, 0)],
            [(Face::D, 0, 0), (Face::L, 2, 2), (Face::F, 2, 0)],
            [(Face::D, 0, 2), (Face::F, 2, 2), (Face::R, 2, 0)],
            [(Face::D, 2, 2), (Face::R, 2, 2), (Face::B, 2, 0)],
            [(Face::D, 2, 0), (Face::B, 2, 2), (Face::L, 2, 0)],
        ];
        let edges = [
            [(Face::F, 0, 1), (Face::U, 2, 1)],
            [(Face::F, 1, 2), (Face::R, 1, 0)],
            [(Face::F, 2, 1), (Face::D, 0, 1)],
            [(Face::F, 1, 0), (Face::L, 1, 2)],
            [(Face::B, 0, 1), (Face::U, 0, 1)],
            [(Face::B, 1, 0), (Face::R, 1, 2)],
            [(Face::B, 2, 1), (Face::D, 2, 1)],
            [(Face::B, 1, 0), (Face::L, 1, 0)],
            [(Face::U, 1, 2), (Face::R, 0, 1)],
            [(Face::U, 1, 0), (Face::L, 0, 1)],
            [(Face::D, 1, 2), (Face::R, 2, 1)],
            [(Face::D, 1, 0), (Face::L, 2, 1)],
        ];
        let c2f = self.color_to_faces();
        let fetch_side = |c| self.fetch_point(c).and_then(|x| c2f.get(&x));
        for [c1, c2, c3] in corners.iter() {
            let sides = (fetch_side(c1), fetch_side(c2), fetch_side(c3));
            if let (Some(side1), Some(side2), Some(side3)) = sides {
                println!("{:?} {:?} {:?}", side1, side2, side3);
            }
        }
    }
}

pub fn read_raw_cube(filename: &str) -> io::Result<RawCube> {
    let lines = read_lines(filename)?;
    let enums = [Face::U, Face::R, Face::F, Face::D, Face::L, Face::B];
    let mut i = 0;
    let mut j = 0;
    let mut colors : [[char; 3]; 3] = [['\0'; 3]; 3];
    let mut faces: HashMap<Face, RawFace> = HashMap::new();
    for line in lines {
        if let Ok(ip) = line {
            let chs: Vec<char> = ip.chars().collect();
            if chs.len() == 3 {
                colors[j][0] = chs[0];
                colors[j][1] = chs[1];
                colors[j][2] = chs[2];
                j += 1;
                if j == 3 {
                    let face = RawFace {
                        colors: colors,
                    };
                    faces.insert(enums[i], face);
                    i += 1;
                    j = 0;
                }
            }
        }
    }
    let cube = RawCube {
        faces: faces,
    };
    Ok(cube)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
