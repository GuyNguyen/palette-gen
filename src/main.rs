use average_color;
use image::RgbImage;
use tokio;

const WIDTH:u32 = 720;
const HEIGHT:u32 = 720;


#[tokio::main]
async fn main() {
    let path: String = String::from("test.jpg");
    let results = average_color::get_average_color(&path).await;
    println!("results: {:?}", results);

    // Assuming `results` is of type Result<Option<Rgb>, String>
    match results {
    Ok(Some(rgb)) => {
        // Convert Rgb to [u8; 3]
        let rgb_array: [u8; 3] = [rgb.r, rgb.g, rgb.b]; // Assuming Rgb has a field that stores the color as an array
        let mut image: RgbImage = RgbImage::new(WIDTH, HEIGHT);
        for pixel in image.enumerate_pixels_mut() {
            *pixel.2 = image::Rgb(rgb_array);
        }

        image.save("output.png").unwrap();
    },
    Ok(None) => {
        // Handle the case where there is no color
        eprintln!("No color found");
    },
    Err(e) => {
        // Handle the error case
        eprintln!("Error retrieving color: {}", e);
    }
}


}
