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
    pub fn init(modem: Modem, ssid: &'static str, psk: &'static str) -> Result<Self, Error> {
        let sysloop = EspSystemEventLoop::take()?;
        let esp_wifi = EspWifi::new(
            modem,
            sysloop.clone(),
            Some(EspDefaultNvsPartition::take()?),
        )?;

        let mut driver = BlockingWifi::wrap(esp_wifi, sysloop.clone())?;

        driver
            .wifi_mut()
            .set_configuration(&Configuration::Client(ClientConfiguration {
                ssid: ssid.into(),
                password: psk.into(),
                auth_method: AuthMethod::WPA2Personal,
                ..Default::default()
            }))?;

        Ok(Wifi { driver })
    }

    pub fn start(&mut self) -> Result<(), Error> {
        debug!("Starting wifi driver");
        self.driver.start()?;

        Ok(())
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        self.driver.connect()?;
        self.driver.wait_netif_up()?;

        debug!(
            "ip info: {:?}",
            self.driver.wifi().sta_netif().get_ip_info()?
        );
        debug!(
            "hostname: {:?}",
            self.driver.wifi().sta_netif().get_hostname()?
        );

        Ok(())
    }
}
