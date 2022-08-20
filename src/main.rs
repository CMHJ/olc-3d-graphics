use olc_pixel_game_engine as olc;
use std::vec::Vec;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub struct Tri<T> {
    p: [V3d<T>; 3],
}

struct Mesh<T> {
    tris: Vec<Tri<T>>,
}

struct OLCEngine3D {
    mesh_cube: Mesh<f32>,
}

impl olc::Application for OLCEngine3D {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        let mut mesh_cube = Mesh::<f32> { tris: Vec::new() };
        mesh_cube.tris = vec![
            // SOUTH
            Tri {
                p: [
                    V3d {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
            },
            Tri {
                p: [
                    V3d {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
            },
            // EAST
            Tri {
                p: [
                    V3d {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
            },
            Tri {
                p: [
                    V3d {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                ],
            },
            // NORTH
            Tri {
                p: [
                    V3d {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
            },
            Tri {
                p: [
                    V3d {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                ],
            },
            // WEST
            Tri {
                p: [
                    V3d {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
            },
            Tri {
                p: [
                    V3d {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
            },
            // TOP
            Tri {
                p: [
                    V3d {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
            },
            Tri {
                p: [
                    V3d {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
            },
            // BOTTOM
            Tri {
                p: [
                    V3d {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
            },
            Tri {
                p: [
                    V3d {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    V3d {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    V3d {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
            },
        ];
        self.mesh_cube = mesh_cube;

        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        // Clears screen and sets black colour.
        olc::clear(olc::BLACK);

        for tri in self.mesh_cube.tris.iter() {

        }
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
        Ok(())
    }
}

fn main() {
    let mut demo = OLCEngine3D {
        mesh_cube: Mesh::<f32> { tris: Vec::new() },
    };
    olc::start("My Game", &mut demo, 256, 240, 4, 4).unwrap();
}
