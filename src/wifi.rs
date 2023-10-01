use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, Wifi as SvcWifi};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};

pub struct Wifi;

impl Wifi {
    pub fn init(modem: Modem, ssid: &'static str, psk: &'static str) -> EspWifi<'static> {
        let mut wifi_driver = EspWifi::new(
            modem,
            EspSystemEventLoop::take().expect("Failed to take system event loop"),
            Some(EspDefaultNvsPartition::take().expect("Failed to take default nvs partition")),
        )
        .expect("Failed to create esp wifi device");

        wifi_driver
            .set_configuration(&Configuration::Client(ClientConfiguration {
                // See .cargo/config.toml to set WIFI_SSID and WIFI_PWD env variables
                ssid: ssid.into(),
                password: psk.into(),
                auth_method: AuthMethod::WPA2Personal,
                ..Default::default()
            }))
            .expect("Failed to set wifi driver configuration");

        wifi_driver
    }

    pub fn start(wifi_driver: &mut EspWifi<'_>) {// TODO: Make it a result type -> Result<()>  {
        wifi_driver.start().expect("Failed to start wifi driver");
        wifi_driver.is_started().expect("Failed to start wifi driver");
        wifi_driver.connect().expect("Failed to initiate connection");
        
        for _ in 0..10 {
            if wifi_driver.is_connected().unwrap() {
                break;
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        if !wifi_driver.is_connected().unwrap() {
            return; // TODO: Error
        }

        for _ in 0..10 {
            if wifi_driver.sta_netif().is_up().unwrap() {
                // Ok(())
                break;
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        

        // Ok(()) // TODO: Err
    }
}