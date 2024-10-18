use xcap::image::RgbaImage;
use xcap::{Monitor, Window};

// pub fn window_screenshot(window_name: String) -> String {
//     let windows = Window::all().unwrap();
//     for window in windows.iter().clone() {
//         if window.is_minimized() {
//             return format!("window {} is minimized", normalized(window.title()));
//         }
//         if window_name == "all" {
//             let image = window.capture_image().unwrap();
//             save_normalized(image, window.title());
//             return "successful".to_string();
//         }
//
//
//     }
//     "window not found".to_string()
// }




pub fn window_screenshot(window_name: String) -> String {
    let windows = Window::all().unwrap();
    for window in windows.iter().clone() {


            let image = window.capture_image().unwrap();
            save_normalized(image, window.title());




    }
    "window not found".to_string()
}



fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}
fn save_normalized(image:RgbaImage, name: &str){
    image.save(format!("images/monitor-{}.png", normalized(name)));
}
pub fn full_screenshot() {
    let monitors = Monitor::all().unwrap();


        let monitors = Monitor::all().unwrap();

        for monitor in monitors {
            let image = monitor.capture_image().unwrap();

            save_normalized(image, monitor.name());

        }
    }

