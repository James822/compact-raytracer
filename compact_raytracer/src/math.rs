use nalgebra as na;

use na::Vector3;

pub fn vec_project(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    let top = a.dot(b);
    let bottom = b.dot(b);
    b.scale(top / bottom)
}
