use xcap::image::RgbaImage;
use xcap::{Monitor, Window};






pub fn window_screenshot(WindowName: Option<String>) -> Vec<RgbaImage> {
    let mut image_paths:Vec<RgbaImage> = vec![];
    let windows = Window::all().unwrap();


        match WindowName {
        None => {
            for window in windows.iter().clone() {
                let image = window.capture_image().unwrap();
                image_paths.push(image);
            }
        }
        Some(_) => {

        }
    }
    image_paths
}




fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}
fn save_normalized(image:RgbaImage, name: &str) -> String{
    image.save(format!("images/monitor-{}.png", normalized(name)));
    return format!("images/monitor-{}.png", normalized(name));
}
pub fn full_screenshot() ->String{
    let mut t = String::new();

        let monitors = Monitor::all().unwrap();

        for monitor in monitors {
            let image = monitor.capture_image().unwrap();

             t = save_normalized(image, monitor.name());

        }
    return t;
    }

