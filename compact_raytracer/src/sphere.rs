use nalgebra as na;

use na::Vector3;

pub struct Sphere {
    pub pos: Vector3<f32>,
    radius: f32,
}

impl Sphere {

    pub fn new(pos: Vector3<f32>, radius: f32) -> Sphere {
	Sphere {
	    pos,
	    radius,
	}
    }

    // origin: origin of the ray being cast
    // dir: direction the ray is facing
    pub fn ray_intersect(&self, origin: &Vector3<f32>, dir: &Vector3<f32>)
		     -> Option<Vector3<f32>>
    {
	let voc = self.pos - origin; // vector from origin to sphere's center

	// if sphere center is behind ray origin
	if voc.dot(dir) <= 0f32 {
	    return None;
	}

	// @TODO: add max render distance checking

	let pc = crate::math::vec_project(&voc, &dir);
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
