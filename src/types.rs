use crate::test_cube::generate_test_cube_mesh;
use olc_pixel_game_engine as olc;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vf3d = V3d<f32>;

impl Vf3d {
    pub fn new() -> Vf3d {
        Vf3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub struct Tri {
    pub p: [Vf3d; 3],
}

impl Tri {
    pub fn new() -> Tri {
        Tri {
            p: [Vf3d::new(); 3],
        }
    }
}

pub struct Mesh {
    pub tris: Vec<Tri>,
}

pub struct Mat4x4 {
    m: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn new() -> Mat4x4 {
        Mat4x4 { m: [[0.0; 4]; 4] }
    }

    pub fn multiply_matrix_vector(i: &Vf3d, o: &mut Vf3d, m: &Mat4x4) {
        o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
        o.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
        o.y = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];
        let w: f32 = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3];

        if w != 0.0f32 {
            o.x /= w;
            o.y /= w;
            o.z /= w;
        }
    }
}

pub struct OLCEngine3D {
    mesh_cube: Mesh,
    mat_proj: Mat4x4,
}

impl OLCEngine3D {
    pub fn new() -> Self {
        OLCEngine3D {
            mesh_cube: Mesh { tris: Vec::new() },
            mat_proj: Mat4x4::new(),
        }
    }
}

impl olc::Application for OLCEngine3D {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        let mesh_cube = generate_test_cube_mesh();
        self.mesh_cube = mesh_cube;

        //Projection Matrix
        let f_near: f32 = 0.1;
        let f_far: f32 = 1000.0;
        let f_fov: f32 = 90.0;
        let f_aspect_ratio: f32 = olc::screen_height() as f32 / olc::screen_width() as f32;
        let f_fov_rad: f32 = 1.0 / (f_fov * 0.5 / 180.0 * 3.14159).tan();

        self.mat_proj.m[0][0] = f_aspect_ratio * f_fov_rad;
        self.mat_proj.m[1][1] = f_fov_rad;
        self.mat_proj.m[2][2] = f_far / (f_far / f_near);
        self.mat_proj.m[3][2] = (-f_far / f_near) / (f_far / f_near);
        self.mat_proj.m[2][3] = 1.0;
        self.mat_proj.m[3][3] = 0.0;

        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        // Clears screen and sets black colour.
        olc::clear(olc::BLACK);

        for tri in self.mesh_cube.tris.iter() {
            let mut tri_proj = Tri::new();
            Mat4x4::multiply_matrix_vector(&tri.p[0], &mut tri_proj.p[0], &self.mat_proj);
            Mat4x4::multiply_matrix_vector(&tri.p[1], &mut tri_proj.p[1], &self.mat_proj);
            Mat4x4::multiply_matrix_vector(&tri.p[2], &mut tri_proj.p[2], &self.mat_proj);

            olc::draw_triangle(
                tri_proj.p[0].x as i32,
                tri_proj.p[0].y as i32,
                tri_proj.p[1].x as i32,
                tri_proj.p[1].x as i32,
                tri_proj.p[2].y as i32,
                tri_proj.p[2].y as i32,
                olc::WHITE);
        }
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
        Ok(())
    }
}
