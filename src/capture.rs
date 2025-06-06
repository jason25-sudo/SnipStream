//! Screen and camera capture stubs
use scrap::{Capturer, Display};
use std::io::ErrorKind;
use std::thread;
use image::{ImageBuffer, Rgba};
use chrono::Local;
use std::path::PathBuf;

pub fn start_capture() {
    println!("[capture] start");
    thread::spawn(|| {
        let display = match Display::primary() {
            Ok(display) => display,
            Err(e) => {
                eprintln!("Failed to get display: {e}");
                return;
            }
        };
        let mut capturer = match Capturer::new(display) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to start capturer: {e}");
                return;
            }
        };
        loop {
            match capturer.frame() {
                Ok(_frame) => {
                    // TODO: encode frame into video output
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // wait for next frame
                    thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(e) => {
                    eprintln!("Capture error: {e}");
                    break;
                }
            }
        }
    });
}

pub fn stop_capture() {
    println!("[capture] stop (not yet implemented)");
    // TODO: implement proper stop signal and finalization
}

/// Capture a single screenshot and save it to `path`.
pub fn save_screenshot(path: &str) {
    println!("[capture] saving screenshot to {path}");
    let display = match Display::primary() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to get display: {e}");
            return;
        }
    };
    let mut capturer = match Capturer::new(display) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to start capturer: {e}");
            return;
        }
    };
    let width = capturer.width();
    let height = capturer.height();
    match capturer.frame() {
        Ok(frame) => {
            // scrap returns BGRA, convert to RGBA
            let mut buf = Vec::with_capacity(width * height * 4);
            for chunk in frame.chunks(4) {
                buf.push(chunk[2]); // R
                buf.push(chunk[1]); // G
                buf.push(chunk[0]); // B
                buf.push(255); // A
            }
            if let Some(image) = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(
                width as u32,
                height as u32,
                buf,
            ) {
                if let Err(e) = image.save(path) {
                    eprintln!("Failed to save screenshot: {e}");
                }
            }
        }
        Err(e) => eprintln!("Failed to capture frame: {e}"),
    }
}

/// Save a screenshot to the `screenshots` directory using a timestamped name.
pub fn save_screenshot_auto() {
    let dir = PathBuf::from("screenshots");
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("Failed to create screenshot directory: {e}");
    }
    let filename = format!(
        "screenshot-{}.png",
        Local::now().format("%Y%m%d-%H%M%S")
    );
    let path = dir.join(filename);
    if let Some(path_str) = path.to_str() {
        save_screenshot(path_str);
    }
}
