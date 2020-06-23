extern crate nalgebra as na;

use na::Vector3;

// epsilon for eq float comparison
const EPSILON: f32 = 0.0001;

pub struct Sphere {
    pos: Vector3<f32>,
    radius: f32,
    color: [u8; 3],
}

fn vec_project(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    let top = a.dot(b);
    let bottom = b.dot(b);
    b.scale(top / bottom)
}

impl Sphere {

    pub fn new(pos: Vector3<f32>, radius: f32, color: [u8; 3]) -> Sphere {
	Sphere {
	    pos,
	    radius,
	    color,
	}
    }

    // origin: origin of the ray being cast
    // dir: direction the ray is facing
    fn ray_intersect(&self, origin: &Vector3<f32>, dir: &Vector3<f32>) -> bool {
	let voc = self.pos - origin; // vector from origin to sphere's center

	// if sphere center is behind ray origin
	if voc.dot(dir) <= 0f32 {
	    return false;
	}

	// @TODO: add max render distance checking

	let pc = vec_project(&voc, &dir);
	let dist_of_center_to_ray = (voc - pc).magnitude();

	if dist_of_center_to_ray > self.radius {
	    return false;
	} else if (dist_of_center_to_ray - self.radius).abs() > EPSILON {
	    return true;
	}
	false
    }
}

pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {

    // @TODO: check to make sure width and height are even,
    // and that params are non-negative and large enough
    pub fn new(width: u32, height: u32) -> WindowSize {
	WindowSize {
	    width,
	    height,
	}
    }
}

fn cast_ray(org: &Vector3<f32>, dir: &Vector3<f32>, spheres: &[Sphere])
	    -> Option<image::Rgb<u8>>
{
    for sphere in spheres.iter() {
	if sphere.ray_intersect(org, dir) {
	    return Some(image::Rgb([0, 255, 255]));
	}
    }
    None
}

fn get_background_pixel() -> image::Rgb<u8> {
    image::Rgb([0, 0, 15])
}

fn render(image_buffer: &mut image::RgbImage, spheres: &[Sphere], window_dim: WindowSize) {
    // cast rays for each pixel with fov
    let fov = 1.0f32;
    let fov_vec = Vector3::new(0.0f32, 0.0f32, fov);
    
    let top_w = window_dim.width;
    let top_h = window_dim.height;
    let ratio = (top_h as f32 / top_w as f32) * 0.5;
    for x in 0..top_w {
	for y in 0..top_h {
	    let cam_x = ((x as f32) / (top_w as f32)) - 0.5f32;
	    let cam_y = ((y as f32) / (top_w as f32)) - ratio;

	    let dir = Vector3::new(cam_x, cam_y, 0.0) - fov_vec;
	    let dir = dir.normalize();
	    
	    let pixel = cast_ray(&fov_vec, &dir, spheres).unwrap_or_else(
		get_background_pixel
	    );
	    
	    image_buffer.put_pixel(x, y, pixel);
	}
    }
}

pub fn render_to_buff(image_buffer: &mut image::RgbImage, window_dim: WindowSize,
		  spheres: Vec<Sphere>)
{
    render(image_buffer, &spheres, window_dim);
}
