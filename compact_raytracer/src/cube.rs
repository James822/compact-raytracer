use nalgebra as na;

use na::Vector3;

// this cube is always aligned to the world axis
// AABB (Axis Aligned Bounding Box)
pub struct Cube {
    pos: Vector3<f32>,
    radius: f32,
}

// only call this if there was a ray intersection
fn get_intersected_point(dir: Vector3<f32>, pos: Vector3<f32>, radius: f32) {
}

impl Cube {

    pub fn new(pos: Vector3<f32>, radius: f32) -> Cube {
	Cube {
	    pos,
	    radius,
	}
    }

    pub fn ray_intersect(&self, origin: &Vector3<f32>, dir: &Vector3<f32>)
//			 -> Option<Vector3<f32>>
    {
    }
}
