use std::vec::Vec;

use olc_pixel_game_engine as olc;
use olc_3d_graphics::types::{OLCEngine3D, Mesh};

fn main() {
    let mut demo = OLCEngine3D {
        mesh_cube: Mesh::<f32> { tris: Vec::new() },
    };
    olc::start("My Game", &mut demo, 256, 240, 4, 4).unwrap();
}
