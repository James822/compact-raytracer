extern crate image;

fn main() {
    let width = 1024;
    let height = 768;

    // create imagebuffer to write our pixels 
    let mut image_buffer = image::RgbImage::new(width, height);

    // loop through and create basic test image
    for x in 0..width {
	for y in 0..height {
	    let i: f32 = (x as f32 / width as f32) * 255f32;
	    let j: f32 = (y as f32 / height as f32) * 255f32;
	    image_buffer.put_pixel(x, y, image::Rgb([i as u8, j as u8, 0]));
	}
    }

    image_buffer.save("render_output/image_test.png").unwrap();
}
