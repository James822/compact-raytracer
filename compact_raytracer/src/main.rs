extern crate compact_raytracer as cr;
extern crate nalgebra as na;

use cr::{WindowSize, Sphere};
use na::Vector3;

fn main() {
    let window_dim = WindowSize::new(1920, 1080);

    // create imagebuffer to write our pixels 
    let mut image_buffer = image::RgbImage::new(window_dim.width,
						window_dim.height);

    let spheres: Vec<Sphere> = create_spheres();

    cr::render_to_buff(&mut image_buffer, window_dim, spheres);

    image_buffer.save("render_output/image_test.png").unwrap();
}

// for now creates a sphere in front of the camera
fn create_spheres() -> Vec<Sphere> {
    let c1: [u8; 3] = [0, 200, 240];
    let s1 = Sphere::new(Vector3::new(0.5, -0.2, -2.0), 0.4, c1);
    let c2: [u8; 3] = [255, 200, 0];
    let s2 = Sphere::new(Vector3::new(-0.4, 0.4, -1.5), 0.3, c2);
    vec![s1, s2]
}

