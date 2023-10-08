use anyhow::{Result, Error, bail};
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

    pub fn start(wifi_driver: &mut EspWifi<'_>) -> Result<(), Error> {
        wifi_driver.start()?;
        wifi_driver.is_started()?;
        wifi_driver.connect()?;
        
        for _ in 0..10 {
            if wifi_driver.is_connected()? {
                break;
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        if !wifi_driver.is_connected()? {
            bail!("Failed to connect to BSS");
        }

        for _ in 0..10 {
            if wifi_driver.sta_netif().is_up()? {
                return Ok(());
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        
        bail!("Failed to bring up network interface");
    }
}