use nalgebra as na;

use rand::prelude::*;
use na::Vector3;
use crate::sphere::Sphere;
use crate::norm_RGB;

fn rand_point_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    loop {
	let p = Vector3::new( (rng.gen::<f32>() * 2.0) - 1.0,
					(rng.gen::<f32>() * 2.0) - 1.0,
					(rng.gen::<f32>() * 2.0) - 1.0 );
	let p_len = p[0]*p[0] + p[1]*p[1] + p[2]*p[2];
	if p_len >= 1.0 {
	    continue;
	}
	return p;
    }
}

fn cast_ray(org: &Vector3<f32>, dir: &Vector3<f32>, bounces: u32, spheres: &[Sphere])
	    -> norm_RGB
{
    let mut smallest_distance = f32::INFINITY;
    let mut intersection_point = Vector3::new(0.0, 0.0, 0.0);
    let mut sphere_pos = Vector3::new(0.0, 0.0, 0.0);

    // find intersection for spheres
    // find intersection for cubes
    
    for sphere in spheres.iter() {
	if let Some(vec) = sphere.ray_intersect(org, dir) {
	    let vec_distance = vec.magnitude();
	    if vec_distance < smallest_distance {
		intersection_point = vec;
		smallest_distance = vec_distance;
		sphere_pos = sphere.pos;
	    }
	}
    }

    if (smallest_distance != f32::INFINITY) && (bounces != 0) {
	let surface_normal = (intersection_point - sphere_pos).normalize();
	let new_dir = (rand_point_in_unit_sphere() + surface_normal).normalize();  
	let color = cast_ray(&intersection_point, &new_dir, bounces - 1, spheres);
	let color = [0.5 * color[0], 0.5 * color[1], 0.5 * color[2]];
	return color;
    }

    get_background_color(&dir)
}

fn get_background_color(ray_dir: &Vector3<f32>) -> norm_RGB {
    // the sky color1
    let sc: norm_RGB = [0.25, 0.55, 0.95];
    let hc: norm_RGB = [0.988, 0.898, 0.69];

    let s = (ray_dir[1] + 1.0) / 2.0;
    
    let a = (s * hc[0]) + ((1.0 - s) * sc[0]);
    let b = (s * hc[1]) + ((1.0 - s) * sc[1]);
    let c = (s * hc[2]) + ((1.0 - s) * sc[2]);

    [a, b, c]
}

fn gamma_correct(pixel: norm_RGB) -> norm_RGB {
    let r = pixel[0];
    let r = r.powf(0.5);
    let g = pixel[1];
    let g = g.powf(0.5);
    let b = pixel[2];
    let b = b.powf(0.5);

    [r, g, b]
}

fn norm_rgb_to_u8(pixel: norm_RGB) -> image::Rgb<u8> {
    let r = pixel[0] * 255f32;
    let g = pixel[1] * 255f32;
    let b = pixel[2] * 255f32;
    image::Rgb([r as u8, g as u8, b as u8])
}

// generates a point in the sqr defined by x, y, and sqr_width.
// (x, y) defines the top left corner of the square
fn random_point_in_sqr(x: f32, y: f32, sqr_width: f32, rng: &mut rand::rngs::ThreadRng)
		       -> (f32, f32)
{
    let rand_x: f32 = rng.gen();
    let rand_y: f32 = rng.gen();
    (x + (rand_x * sqr_width), y + (rand_y * sqr_width))
}

// the cam_x and cam_y values here are the top left of the current pixel this function
// is working on
fn process_pixel(cam_x: f32, cam_y: f32, cam_pixel_width: f32, samples: u32,
		 origin: Vector3<f32>, fov_vec: Vector3<f32>, bounces: u32,
		 spheres: &[Sphere])
		 -> norm_RGB
{
    // for later
    let samples_f32: f32 = samples as f32;

    let mut main_color: norm_RGB = [0.0, 0.0, 0.0];
    let grid_width_count: u32 = (samples_f32).sqrt() as u32;

    // width of the individual grid squares
    let grid_width = cam_pixel_width / (grid_width_count as f32);

    // caching thread_rng for performance
    let mut rng = rand::thread_rng();

    for x in 0..grid_width_count {
	for y in 0..grid_width_count {

	    let top_left_x = cam_x + ( grid_width * (x as f32) );
	    let top_left_y = cam_y + ( grid_width * (y as f32) );
	    
	    let (ray_x, ray_y) = random_point_in_sqr(top_left_x, top_left_y,
						     grid_width, &mut rng);

	    let dir = Vector3::new(ray_x, ray_y, 0.0) - fov_vec;
	    let dir = dir.normalize();
	    let color = cast_ray(&origin, &dir, bounces, spheres);
	    main_color[0] += color[0];
	    main_color[1] += color[1];
	    main_color[2] += color[2];
	}
    }
    main_color[0] /= samples_f32;
    main_color[1] /= samples_f32;
    main_color[2] /= samples_f32;

    main_color
}

// this function mostly setups up the exact coordiantes from where rays should be shot
// and then drawn to a pixel, the actual ray formation with fov happens in process_pixel()
fn render(image_buffer: &mut image::RgbImage, window_dim: crate::WindowSize, samples: u32,
	  bounces: u32, spheres: &[Sphere])
{
    // setting up fov
    let fov: f32 = 0.45;
    let fov_vec = Vector3::new(0.0, 0.0, fov);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let width = window_dim.width;
    let height = window_dim.height;

    // @E
    // setting the virtual camera dimensions (with correct aspect ratio)
    // the width is always 1.0 with this
    let cam_width: f32 = 1.0;
    let cam_height: f32 = (height as f32) / (width as f32);
    let cam_width = cam_width - 0.5;
    let cam_height = cam_height - (cam_height / 2.0);
    // @EE

    // calculating pixel length of the virtual camera
    let pixel_length = 1.0 / (width as f32);

    for x in 0..width {
	for y in 0..height {
	    let cam_x = ( (x as f32) * pixel_length ) - cam_width;
	    let cam_y = ( (y as f32) * pixel_length ) - cam_height;
	    
	    let raw_pixel = process_pixel(cam_x, cam_y, pixel_length,
					  samples, origin, fov_vec, bounces, spheres);
	    let gamma_corrected_pixel = gamma_correct(raw_pixel);

	    image_buffer.put_pixel(x, y, norm_rgb_to_u8(gamma_corrected_pixel))
	}
    }
}

pub fn render_to_buff(image_buffer: &mut image::RgbImage, window_dim: crate::WindowSize,
		      samples: u32, bounces: u32, spheres: Vec<Sphere>)
{
    render(image_buffer, window_dim, samples, bounces, &spheres);
}
