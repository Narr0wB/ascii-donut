use std::io::{stdout, Write};
use termion::cursor;

use nalgebra::{*};

const ASCII: &str = "`.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

pub struct Renderer {
    znear: f32,
    zfar: f32,
    rows: u16,
    cols: u16,
    framebuffer: Vec<u8>
}


pub fn map_range(i: u32, max: u32, range_start: f32, range_end: f32) -> f32 {
    (range_end - range_start) * (i as f32 / max as f32)
}

pub fn create_toroid(hole_radius: f32, thickness_radius: f32, resolution: u32, resolution2: u32) -> Vec<(Vector3<f32>, Vector3<f32>)> {
    let mut result = vec![];

    let internal_radius = hole_radius + thickness_radius;

    for i in 0..resolution2 {
        let theta = map_range(i, resolution2, 0.0, 2.0 * std::f32::consts::PI);
        let rotation = Rotation3::from_axis_angle(&Vector3::y_axis(), theta);

        for j in 0..resolution {
            let phi = map_range(j, resolution, 0.0, 2.0 * std::f32::consts::PI);

            let circle = Vector3::new(internal_radius + thickness_radius * phi.sin(), thickness_radius * phi.cos(), 0.0);
            let normal = Vector3::new(phi.sin(), phi.cos(), 0.0);
            
            result.push((rotation.transform_vector(&circle), rotation.transform_vector(&normal)));
        }
    }

    result
}

pub fn create_cube(length: f32, resolution: u32) -> Vec<Vector3<f32>> {
    let mut vector = vec![];

    for i in 0..resolution {
        let y = map_range(i, resolution, 0.0, length);

        for j in 0..resolution {
            let x = map_range(j, resolution, 0.0, length);

            for k in 0..resolution {
                let z = map_range(k, resolution, 0.0, length);

                vector.push(Vector3::new(x, y, z));
            }
        }
    }

    vector
}

impl Renderer {
    pub fn new(znear: f32, zfar: f32, rows: u16, cols: u16) -> Self {
        Self {
            znear, zfar, rows, cols, framebuffer: vec![b' '; (rows*cols) as usize]
        }
    }

    pub fn render_verices(&mut self, vertices: &mut Vec<Vector3<f32>>) {
        self.clear_buffer();

        vertices
            .sort_by(|a, b| b.z.total_cmp(&a.z));

        for vertex in vertices.iter() {
            let aspect_ratio = 16.0 / 9.0;

            let y_proj = (self.znear / vertex.z) * vertex.y * aspect_ratio;
            let x_proj = (self.znear / vertex.z) * vertex.x;
            
            let x_coord = ((x_proj * self.cols as f32) + (self.cols as f32) / 2.0) as u16;
            let y_coord = ((y_proj * self.rows as f32) + (self.rows as f32) / 2.0) as u16; 

            let luminance = (vertex.z - self.znear) / (self.zfar - self.znear) * 255.0;

            if x_coord < self.cols && y_coord < self.rows {
                let index = ((self.rows*self.cols) - ((y_coord) * self.cols) - (self.cols - x_coord)) as usize;
                self.framebuffer[index] = self.convert_to_ascii(luminance);
            }
        }
    }

    pub fn render_vertices_light(&mut self, vertices: &mut Vec<(Vector3<f32>, Vector3<f32>)>, light_direction: Vector3<f32>) {
        self.clear_buffer();

        vertices
            .sort_by(|a, b| b.0.z.total_cmp(&a.0.z));

        for (vertex, normal) in vertices.iter() {
            let aspect_ratio = 16.0 / 9.0;

            let y_proj = (self.znear / vertex.z) * vertex.y * aspect_ratio;
            let x_proj = (self.znear / vertex.z) * vertex.x;
            
            let x_coord = ((x_proj * self.cols as f32) + (self.cols as f32) / 2.0) as u16;
            let y_coord = ((y_proj * self.rows as f32) + (self.rows as f32) / 2.0) as u16; 

            let luminance = light_direction.dot(&normal);

            if x_coord < self.cols && y_coord < self.rows {
                let index = ((self.rows*self.cols) - ((y_coord) * self.cols) - (self.cols - x_coord)) as usize;
                self.framebuffer[index] = self.convert_to_ascii(if luminance > 0.0 { luminance } else { 0.0 });
            }
        }
    }

    pub fn print_buffer(&self) {
        let mut stdout = stdout().lock();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        for (i, &c) in self.framebuffer.iter().enumerate() {
            if i > 0 && i % self.cols as usize == 0 {
                write!(stdout, "\r\n").unwrap();
            }
            write!(stdout, "{}", c as char).unwrap();
        }

        stdout.flush().unwrap();
    }

    fn clear_buffer(&mut self) {
        self.framebuffer.clear();
        self.framebuffer = vec![b' '; (self.rows*self.cols) as usize];
    }
    
    fn convert_to_ascii(&self, luminance: f32) -> u8 {
        let len = ASCII.len() as f32;
        let index = (luminance * (len - 1.0)).clamp(0.0, len - 1.0);

        ASCII.as_bytes()[index as usize]
    }
}