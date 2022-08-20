use olc_pixel_game_engine as olc;

use crate::test_cube::generate_test_cube_mesh;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub struct Tri<T> {
    pub p: [V3d<T>; 3],
}

pub struct Mesh<T> {
    pub tris: Vec<Tri<T>>,
}

pub struct OLCEngine3D {
    pub mesh_cube: Mesh<f32>,
}

impl olc::Application for OLCEngine3D {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        let mesh_cube = generate_test_cube_mesh();
        self.mesh_cube = mesh_cube;

        //Projection Matrix

        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        // Clears screen and sets black colour.
        olc::clear(olc::BLACK);

        for tri in self.mesh_cube.tris.iter() {}
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
        Ok(())
    }
}
