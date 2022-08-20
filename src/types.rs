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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
        o.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];
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
    f_theta: f32,
}

impl OLCEngine3D {
    pub fn new() -> Self {
        OLCEngine3D {
            mesh_cube: Mesh { tris: Vec::new() },
            mat_proj: Mat4x4::new(),
            f_theta: 0.0,
        }
    }
}

impl olc::Application for OLCEngine3D {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        self.mesh_cube = generate_test_cube_mesh();

        //Projection Matrix
        let f_near: f32 = 0.1;
        let f_far: f32 = 1000.0;
        let f_fov: f32 = 90.0;
        let f_aspect_ratio: f32 = olc::screen_height() as f32 / olc::screen_width() as f32;
        let f_fov_rad: f32 = 1.0 / (f_fov * 0.5 / 180.0 * 3.14159).tan();

        self.mat_proj.m[0][0] = f_aspect_ratio * f_fov_rad;
        self.mat_proj.m[1][1] = f_fov_rad;
        self.mat_proj.m[2][2] = f_far / (f_far - f_near);
        self.mat_proj.m[3][2] = (-f_far * f_near) / (f_far - f_near);
        self.mat_proj.m[2][3] = 1.0;
        self.mat_proj.m[3][3] = 0.0;

        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        // Clears screen and sets black colour.
        olc::clear(olc::BLACK);

        let mut mat_rot_z = Mat4x4::new();
        let mut mat_rot_x = Mat4x4::new();
        self.f_theta += 1.0 * _elapsed_time;
        if self.f_theta.is_infinite() {
            self.f_theta = f32::MIN;
        }

        // Rotation Z
        mat_rot_z.m[0][0] = self.f_theta.cos();
        mat_rot_z.m[0][1] = self.f_theta.sin();
        mat_rot_z.m[1][0] = -self.f_theta.sin();
        mat_rot_z.m[1][1] = self.f_theta.cos();
        mat_rot_z.m[2][2] = 1.0;
        mat_rot_z.m[3][3] = 1.0;

        // Rotation X
        mat_rot_x.m[0][0] = 1.0;
        mat_rot_x.m[1][1] = (self.f_theta * 0.5).cos();
        mat_rot_x.m[1][2] = (self.f_theta * 0.5).sin();
        mat_rot_x.m[2][1] = -(self.f_theta * 0.5).sin();
        mat_rot_x.m[2][2] = (self.f_theta * 0.5).cos();
        mat_rot_x.m[3][3] = 1.0;

        // Draw triangles
        for tri in self.mesh_cube.tris.iter() {
            let mut tri_proj = Tri::new();
            let mut tri_rot_z = Tri::new();
            let mut tri_rot_zx = Tri::new();

            Mat4x4::multiply_matrix_vector(&tri.p[0], &mut tri_rot_z.p[0], &mat_rot_z);
            Mat4x4::multiply_matrix_vector(&tri.p[1], &mut tri_rot_z.p[1], &mat_rot_z);
            Mat4x4::multiply_matrix_vector(&tri.p[2], &mut tri_rot_z.p[2], &mat_rot_z);

            Mat4x4::multiply_matrix_vector(&tri_rot_z.p[0], &mut tri_rot_zx.p[0], &mat_rot_x);
            Mat4x4::multiply_matrix_vector(&tri_rot_z.p[1], &mut tri_rot_zx.p[1], &mat_rot_x);
            Mat4x4::multiply_matrix_vector(&tri_rot_z.p[2], &mut tri_rot_zx.p[2], &mat_rot_x);

            let mut tri_trans = tri_rot_zx;
            tri_trans.p[0].z = tri_rot_zx.p[0].z + 3.0;
            tri_trans.p[1].z = tri_rot_zx.p[1].z + 3.0;
            tri_trans.p[2].z = tri_rot_zx.p[2].z + 3.0;

            Mat4x4::multiply_matrix_vector(&tri_trans.p[0], &mut tri_proj.p[0], &self.mat_proj);
            Mat4x4::multiply_matrix_vector(&tri_trans.p[1], &mut tri_proj.p[1], &self.mat_proj);
            Mat4x4::multiply_matrix_vector(&tri_trans.p[2], &mut tri_proj.p[2], &self.mat_proj);

            // Scale into view
            tri_proj.p[0].x += 1.0;
            tri_proj.p[0].y += 1.0;
            tri_proj.p[1].x += 1.0;
            tri_proj.p[1].y += 1.0;
            tri_proj.p[2].x += 1.0;
            tri_proj.p[2].y += 1.0;

            tri_proj.p[0].x *= 0.5 * olc::screen_width() as f32;
            tri_proj.p[0].y *= 0.5 * olc::screen_height() as f32;
            tri_proj.p[1].x *= 0.5 * olc::screen_width() as f32;
            tri_proj.p[1].y *= 0.5 * olc::screen_height() as f32;
            tri_proj.p[2].x *= 0.5 * olc::screen_width() as f32;
            tri_proj.p[2].y *= 0.5 * olc::screen_height() as f32;

            olc::draw_triangle(
                tri_proj.p[0].x as i32,
                tri_proj.p[0].y as i32,
                tri_proj.p[1].x as i32,
                tri_proj.p[1].y as i32,
                tri_proj.p[2].x as i32,
                tri_proj.p[2].y as i32,
                olc::WHITE,
            );
        }
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
        Ok(())
    }
}
