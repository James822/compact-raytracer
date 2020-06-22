extern crate image;
extern crate nalgebra as na;

use na::Vector3;

// epsilon for eq float comparison
const EPSILON: f32 = 0.0001;

struct Sphere {
    pos: Vector3<f32>,
    radius: f32,
}

fn vec_project(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    let top = a.dot(b);
    let bottom = b.dot(b);
    b.scale(top / bottom)
}

impl Sphere {

    fn new(pos: Vector3<f32>, radius: f32) -> Sphere {
	Sphere {
	    pos,
	    radius,
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

struct WindowSize {
    width: u32,
    height: u32,
}

impl WindowSize {

    // @TODO: check to make sure width and height are even,
    // and that params are non-negative and large enough
    fn new(width: u32, height: u32) -> WindowSize {
	WindowSize {
	    width,
	    height,
	}
    }
}

fn main() {
    let window_dim = WindowSize::new(1024, 768);

    // create imagebuffer to write our pixels 
    let mut image_buffer = image::RgbImage::new(window_dim.width,
						window_dim.height);

    render_to_buff(&mut image_buffer, window_dim);

    image_buffer.save("render_output/image_test.png").unwrap();
}

fn cast_ray(org: &Vector3<f32>, dir: &Vector3<f32>, spheres: &[Sphere]) -> image::Rgb<u8> {
    for sphere in spheres.iter() {
	if sphere.ray_intersect(org, dir) {
	    return image::Rgb([0, 255, 255]);
	}
    }
    image::Rgb([0, 0, 0])
}

// for now creates a sphere in front of the camera
fn create_spheres() -> Vec<Sphere> {
    vec![Sphere::new(Vector3::new(0.0, 0.0, -4.0), 0.2)]
}

fn render(image_buffer: &mut image::RgbImage, spheres: &[Sphere], window_dim: WindowSize) {
    // cast rays for each pixel with fov
    let top_w = window_dim.width;
    let top_h = window_dim.height;
    for x in 0..top_w {
	for y in 0..top_h {
	    // for now it is an orthographic render
	    let ph_dir = Vector3::new(0.0, 0.0, -1.0);
	    let ray_x = ((x as f32) / (top_w as f32)) - 0.5f32;
	    let ray_y = ((y as f32) / (top_w as f32)) - 0.5f32;
	    let origin = Vector3::new(ray_x, ray_y,  0.0);
	    image_buffer.put_pixel(x, y, cast_ray(&origin, &ph_dir, spheres));
	}
    }
}

fn render_to_buff(image_buffer: &mut image::RgbImage, window_dim: WindowSize) {
    let spheres: Vec<Sphere> = create_spheres();
    render(image_buffer, &spheres, window_dim);
}
