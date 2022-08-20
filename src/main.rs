use olc_pixel_game_engine as olc;
use olc_3d_graphics_rs::types::OLCEngine3D;

fn main() {
    let mut demo = OLCEngine3D::new();
    olc::start("olc Engine 3D", &mut demo, 256, 240, 4, 4).unwrap();
}
