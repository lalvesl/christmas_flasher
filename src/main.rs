use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};
// use esp_idf_sys as _;
use heapless::String;
use std::{thread::sleep, time::Duration};

fn main() {
    esp_idf_hal::sys::link_patches(); //Needed for esp32-rs
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Entered Main function!");
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs)).unwrap();

    let mut ssid: String<32> = String::try_from(include_str!("../.env/WIFI_SSID")).unwrap();
    let mut password: String<64> = String::try_from(include_str!("../.env/WIFI_PASSWORD")).unwrap();

    ssid.truncate(ssid.len() - 1);
    password.truncate(password.len() - 1);

    wifi_driver
        .set_configuration(&Configuration::Client(ClientConfiguration {
            ssid,
            password,
            ..Default::default()
        }))
        .unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();
    while !wifi_driver.is_connected().unwrap() {
        let config = wifi_driver.get_configuration().unwrap();
        log::info!("Waiting for station {:?}", config);
        sleep(Duration::from_millis(250));
    }
    log::info!("Should be connected now");
    loop {
        log::info!(
            "IP info: {:?}",
            wifi_driver.sta_netif().get_ip_info().unwrap()
        );
        sleep(Duration::new(10, 0));
    }
}
