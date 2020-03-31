use std::env;

mod representation;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    if let Ok(cube) = representation::read_raw_cube(filename) {
        println!("{:?}", cube);
        println!("{:#?}", cube.face_colors());
        println!("{:#?}", cube.color_to_faces());
        cube.facelet();
   }
}
