use anyhow::Result;

use esp32_nimble::BLEDevice;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;
use espcam::ble::ble_advertise_config;
use espcam::espcam::Camera;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let camera = Camera::new(
        peripherals.pins.gpio32,
        peripherals.pins.gpio0,
        peripherals.pins.gpio5,
        peripherals.pins.gpio18,
        peripherals.pins.gpio19,
        peripherals.pins.gpio21,
        peripherals.pins.gpio36,
        peripherals.pins.gpio39,
        peripherals.pins.gpio34,
        peripherals.pins.gpio35,
        peripherals.pins.gpio25,
        peripherals.pins.gpio23,
        peripherals.pins.gpio22,
        peripherals.pins.gpio26,
        peripherals.pins.gpio27,
        esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG,
        esp_idf_sys::camera::framesize_t_FRAMESIZE_UXGA,
    )
    .unwrap();
    let ble_device = BLEDevice::take();
    let ble_advertising = ble_device.get_advertising();
    let server = ble_device.get_server();
    ble_advertise_config("EPS32", server, ble_advertising);
    ble_advertising.lock().start()?;
    loop {
        let framebuffer = camera.get_framebuffer();

        if let Some(framebuffer) = framebuffer {
            println!("Got framebuffer!");
            println!("width: {}", framebuffer.width());
            println!("height: {}", framebuffer.height());
            println!("len: {}", framebuffer.data().len());
            println!("format: {}", framebuffer.format());
            FreeRtos::delay_ms(1000);
        } else {
            log::info!("no framebuffer");
        }
        FreeRtos::delay_ms(10);
    }
}
