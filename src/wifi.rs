use anyhow::{Error, Result};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi},
};
use log::*;

pub struct Wifi {
    pub driver: BlockingWifi<EspWifi<'static>>,
}

impl Wifi {
    pub fn init(modem: Modem, ssid: &'static str, psk: &'static str) -> Self {
        let sysloop = EspSystemEventLoop::take().expect("Failed to take system event loop");
        let esp_wifi = EspWifi::new(
            modem,
            sysloop.clone(),
            Some(EspDefaultNvsPartition::take().expect("Failed to take default nvs partition")), // ? Necessary?
        )
        .expect("Failed to create esp wifi device");

        let mut driver = BlockingWifi::wrap(esp_wifi, sysloop.clone())
            .expect("Failed to intialise BlockingWifi object");

        driver
            .wifi_mut()
            .set_configuration(&Configuration::Client(ClientConfiguration {
                ssid: ssid.into(),
                password: psk.into(),
                auth_method: AuthMethod::WPA2Personal,
                ..Default::default()
            }))
            .expect("Failed to set wifi driver configuration");

        Wifi { driver }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.driver.start()?;
        self.driver.connect()?;
        self.driver.wait_netif_up()?;

        let ip = self.driver.wifi().sta_netif().get_ip_info()?;
        info!("IP info: {:?}", ip);

        Ok(())
    }
}
