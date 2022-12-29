use image::buffer::ConvertBuffer;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat, Rgb, RgbImage};
use oidn;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use rand::Rng;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::Cursor;
use std::path::Path;

fn main() {
    watercolorize("D:/Odds and Ends/images".to_string());
}

// To be deprecated
pub fn add_one(original: u32) -> u32 {
    return original + 1;
}

pub fn watercolorize(folderpath: String) {
    // Get an iterator over the entries in the directory
    let directory_path = Path::new(&folderpath[..]);
    let entries = fs::read_dir(directory_path).unwrap();

    // Filter the iterator to only include .jpg and .png files
    let filepaths: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension() == Some("jpg".as_ref()) || path.extension() == Some("png".as_ref())
            {
                Some(path.to_str().unwrap().to_string())
            } else {
                None
            }
        })
        .collect();

    for filepath in filepaths {
        // Open the image file in the directory
        let image_path = Path::new(&filepath[..]);
        let image_bytes = fs::read(image_path).unwrap();

        // Decode the image file into a DynamicImage
        let image = image::load_from_memory_with_format(&image_bytes, ImageFormat::JPEG).unwrap();

        // Convert the image to a Vec<f32> in RGB
        let image_data: Vec<f32> = image.rgb_f32().data.iter().map(|&x| x as f32).collect();
        let mut filter_output = vec![0.0f32; image_data.len()];

        let device = oidn::Device::new();
        oidn::RayTracing::new(&device)
            // Optionally add float3 normal and albedo buffers as well
            .srgb(true)
            .image_dimensions(input.width() as usize, input.height() as usize)
            .filter(&image_data[..], &mut filter_output[..])
            .expect("Filter config error!");

        if let Err(e) = device.get_error() {
            println!("Error denosing image: {}", e.1);
        }

        // Create an RgbImage from the Vec<f32>
        let mut image = RgbImage::new(2, 2);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let i = (y * 2 + x) as usize;
            *pixel = Rgb([
                filter_output[i * 3],
                filter_output[i * 3 + 1],
                filter_output[i * 3 + 2],
            ]);
        }

        // Convert the RgbImage to a DynamicImage
        let image = DynamicImage::ImageRgb8(image);

        // Save the image to a file
        let image_path = Path::new("./image-edited.jpg");
        image.save(image_path).unwrap();
    }
}

fn make_guide_text() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    let mut guess: String;
    let mut guess_number: u32;

    loop {
        guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse() {
            Ok(number) => guess_number = number,
            Err(_error) => {
                guess = "0".to_string();
                guess_number = 0;
            }
        };

        println!("You guessed: {guess}");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn test_python() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}
