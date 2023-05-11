use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{Write, stdout};

use gif::EncodingError as GifEncodingError;
use image::ImageError;
use thirtyfour::prelude::*;

use url2gif::Settings;

#[tokio::main]
async fn main() -> Result<(), MyError> {
    let settings = Settings::from_args();
    // Set up WebDriver session
    let mut caps = DesiredCapabilities::chrome();

    if settings.headless {
        caps.set_headless().unwrap();
    }
    let driver = WebDriver::new("http://localhost:9515", caps).await.expect("Failed to start WebDriver session");
    println!("got driver");

    // Load the website and wait for it to load
    driver.goto(settings.target).await.unwrap();
    println!("loaded");
    driver.find(By::Tag("body")).await.unwrap();


    // Get the size of the webpage
    let width = driver.execute("return window.innerWidth", vec![]).await?.json().as_i64().unwrap() as u16;
    let height = driver.execute("return window.innerHeight", vec![]).await?.json().as_i64().unwrap() as u16;
    println!("width {} height {}", width, height);

    // Create GIF encoder
    let mut gif_encoder = gif::Encoder::new(File::create(settings.output_filename).unwrap(), width, height, &[])?;

    let mut stdout = stdout();

    // Capture screenshots at regular intervals and add them to the GIF
    for _i in 0..settings.count {
        print!("\rProcessing frame {}/{}", _i + 1, settings.count);
        stdout.flush().unwrap();
        let screenshot_data = driver.screenshot_as_png().await?;
        image::load_from_memory_with_format(&screenshot_data, image::ImageFormat::Png)?;
        let screenshot_image = image::load_from_memory(&screenshot_data)?;
        let frame = gif::Frame::from_rgba_speed(width, height, &mut screenshot_image.to_rgba8(), 30);
        gif_encoder.write_frame(&frame)?;
    }

    Ok(())
}

#[derive(Debug)]
enum MyError {
    WDE(WebDriverError),
    GE(GifEncodingError),
    IE(ImageError)
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl Error for MyError {}

impl From<WebDriverError> for MyError {
    fn from(value: WebDriverError) -> Self {
        Self::WDE(value)
    }
}

impl From<GifEncodingError> for MyError {
    fn from(value: GifEncodingError) -> Self {
        Self::GE(value)
    }
}

impl From<ImageError> for MyError {
    fn from(value: ImageError) -> Self {
        Self::IE(value)
    }
}
