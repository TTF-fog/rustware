use xcap::Monitor;
use crate::screenshot_all::window_screenshot;

pub mod screenshot_all;


fn main() {
    screenshot_all::full_screenshot();
    println!("{:?}", window_screenshot("all".to_string()));
}
