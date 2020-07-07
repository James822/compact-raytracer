use nalgebra as na;

pub mod math;
pub mod sphere;
mod rendering;

use std::env;
use std::process;
use na::Vector3;
use sphere::Sphere;

// EPSILON for comparison, the f32::EPSILON is too strict
const EPSILON: f32 = 0.00001;

type norm_RGB = [f32; 3];

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

// prints all the relevant render info and settings that the user chose
fn print_render_info(res: &WindowSize, spp: u32, bounces: u32, output_file: &str) {
    println!("Render profile: ");
    println!("resolution / window size: {} x {}", res.width, res.height);
    println!("samples per pixel: {}", spp);
    println!("max ray bounces: {}", bounces);
    println!("output file (in /compact_raytracer/render_output): \"{}\"", output_file);
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args[1] == "help" {
	println!("help urself");
	process::exit(0);
    }
    
    if args.len() != 4 {
	println!("ERROR: incorrect amount of arguments, type \"cargo run --release help\"");
	process::exit(1);
    }

    let x_res: u32 = args[1].parse().unwrap();
    let y_res: u32 = args[2].parse().unwrap();
    let window_dim = WindowSize::new(x_res, y_res);
    let samples_per_pixel: u32 = args[3].parse().unwrap();
    // @TODO make these arguments
    let render_file_name = "image_test.png";
    let max_bounces: u32 = 45; 
    // @END_TODO

    print_render_info(&window_dim, samples_per_pixel, max_bounces, render_file_name);

    // create imagebuffer to write our pixels 
    let mut image_buffer = image::RgbImage::new(window_dim.width,
						window_dim.height);

    let spheres: Vec<Sphere> = create_spheres();

    rendering::render_to_buff(&mut image_buffer, window_dim, samples_per_pixel,
			      max_bounces, spheres);

    let output_file = format!("render_output/{}", render_file_name);
    image_buffer.save(output_file).unwrap();
}

// for now sphere positions and sizes are hardcoded
// @TODO make it possible to define where spheres are and their radius
fn create_spheres() -> Vec<Sphere> {
    let s1 = Sphere::new(Vector3::new(0.5, -0.2, -2.0), 0.4);
    let s2 = Sphere::new(Vector3::new(-0.4, 0.4, -1.5), 0.3);
    let s3 = Sphere::new(Vector3::new(-0.3, 0.3, -5.0), 0.9);
    vec![s1, s2, s3]
}

// fn create_cubes() -> Vec<Cube> {
//     // stuff
// }
