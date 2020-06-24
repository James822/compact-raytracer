extern crate nalgebra as na;

use na::Vector3;

// EPSILON for comparison, the f32::EPSILON is too strict
// const EPSILON: f32 = 0.00001;

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
    fn ray_intersect(&self, origin: &Vector3<f32>, dir: &Vector3<f32>)
		     -> Option<Vector3<f32>>
    {
	let voc = self.pos - origin; // vector from origin to sphere's center

	// if sphere center is behind ray origin
	if voc.dot(dir) <= 0f32 {
	    return None;
	}

	// @TODO: add max render distance checking

	let pc = vec_project(&voc, &dir);
	let vec_c_to_ray = pc - voc;
	let dist_of_center_to_ray = vec_c_to_ray.magnitude();

	if dist_of_center_to_ray > self.radius {
	    None
	} else {
	    let asqr = self.radius*self.radius;
	    let b = pc - self.pos;
	    let bsqr = (&b).dot(&b);
	    let di1 = (pc - origin).magnitude() - (asqr - bsqr);
	    let ans = origin + dir.scale(di1);
	    Some(ans) 
	}
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

fn color_point(point: Vector3<f32>, sphere_pos: Vector3<f32>, ray_dir: Vector3<f32>)
	       -> image::Rgb<u8>
{
    let n = (point - sphere_pos).normalize(); // normal vector of sphere
    let color = {
	let view_dir = -ray_dir;
	let ans = n.dot(&view_dir);
	if ans < 0.0 {
	    0.0
	} else {
	    ans
	}
    };
    let color = color * 255f32;
    image::Rgb([color as u8, color as u8, color as u8])
}

fn cast_ray(org: &Vector3<f32>, dir: &Vector3<f32>, spheres: &[Sphere])
	    -> Option<image::Rgb<u8>>
{
    for sphere in spheres.iter() {
	if let Some(vec) = sphere.ray_intersect(org, dir) {
	    let color = color_point(vec, sphere.pos, *dir);
	    return Some(color);
	}
    }
    None
}

fn get_background_pixel() -> image::Rgb<u8> {
    image::Rgb([0, 230, 230])
}

fn gamma_correct(pixel: image::Rgb<u8>) -> image::Rgb<u8> {
    let r = pixel[0] as f32;
    let r = r / 255f32;
    let r = r.powf(0.5);
    let g = pixel[1] as f32;
    let g = g / 255f32;
    let g = g.powf(0.5);
    let b = pixel[2] as f32;
    let b = b / 255f32;
    let b = b.powf(0.5);

    let r = r * 255f32;
    let g = g * 255f32;
    let b = b * 255f32;

    image::Rgb([r as u8, g as u8, b as u8])
}

fn render(image_buffer: &mut image::RgbImage, spheres: &[Sphere], window_dim: WindowSize) {
    // cast rays for each pixel with fov
    let fov = 0.5f32;
    let fov_vec = Vector3::new(0.0f32, 0.0f32, fov);
    let origin = Vector3::new(0.0, 0.0, 0.0);
    
    let top_w = window_dim.width;
    let top_h = window_dim.height;
    let ratio = (top_h as f32 / top_w as f32) * 0.5;
    for x in 0..top_w {
	for y in 0..top_h {
	    let cam_x = ((x as f32) / (top_w as f32)) - 0.5f32;
	    let cam_y = ((y as f32) / (top_w as f32)) - ratio;

	    let dir = Vector3::new(cam_x, cam_y, 0.0) - fov_vec;
	    let dir = dir.normalize();

	    let raw_pixel = cast_ray(&origin, &dir, spheres).unwrap_or_else(
		get_background_pixel
	    );
	    let gamma_corrected_pixel = gamma_correct(raw_pixel);
	    
	    image_buffer.put_pixel(x, y, gamma_corrected_pixel);
	}
    }
}

pub fn render_to_buff(image_buffer: &mut image::RgbImage, window_dim: WindowSize,
		  spheres: Vec<Sphere>)
{
    render(image_buffer, &spheres, window_dim);
}
