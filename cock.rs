fn main() {
    println!("Hello World!");
}

use show_image::{ImageView, ImageInfo, create_window};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

  let image = ImageView::new(ImageInfo::rgb8(1920, 1080), pixel_data);

  // Create a window with default options and display the image.
  let window = create_window("image", Default::default())?;
  window.set_image("image-001", image)?;

  Ok(())
}