use anyhow::{Error, Result};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi},
};
use log::*;
use std::time::Instant;

use crate::led::{RGB8, WS2812RMT};

pub struct Wifi<'a> {
    led: WS2812RMT<'a>,
    connected: bool,
    lost_connection_at: Option<Instant>,
    pub driver: BlockingWifi<EspWifi<'static>>,
}

impl Wifi<'_> {
    pub fn init<'a>(
        modem: Modem,
        ssid: &'static str,
        psk: &'static str,
        led: WS2812RMT<'a>,
    ) -> Result<Wifi<'a>, Error> {
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

        let mut wifi = Wifi {
            led,
            connected: false,
            lost_connection_at: None,
            driver,
        };
        wifi.set_connection(false)?;

        Ok(wifi)
    }

    pub fn start(&mut self) -> Result<(), Error> {
        debug!("Starting wifi driver");
        self.driver.start()?;

        Ok(())
    }

    fn set_connection(&mut self, is_connected: bool) -> Result<(), Error> {
        self.connected = is_connected;

        if self.connected {
            self.lost_connection_at = None;
            self.led.set_pixel(RGB8::new(0, 100, 0))?;
        } else {
            self.lost_connection_at = Some(Instant::now());
            self.led.set_pixel(RGB8::new(100, 0, 0))?;
        }

        Ok(())
    }

    pub fn wait_for_connection(&mut self) -> Result<(), Error> {
        if self.is_up() {
            return Ok(());
        }

        loop {
            info!("Scanning for AP");
            self.driver.connect()?;
            if self.driver.wait_netif_up().is_ok() {
                break;
            }
        }

        debug!(
            "ip info: {:?}",
            self.driver.wifi().sta_netif().get_ip_info()?
        );
        debug!(
            "hostname: {:?}",
            self.driver.wifi().sta_netif().get_hostname()?
        );

        self.set_connection(true)?;
        Ok(())
    }

    pub fn poll(&mut self) -> Result<(), Error> {
        if self.is_up() {
            if !self.connected {
                self.set_connection(true)?;
                info!(
                    "Reestablished connection: {:?}",
                    self.driver.wifi().sta_netif().get_ip_info()?.ip
                );
            }
        } else if self.connected
            || self
                .lost_connection_at
                .is_some_and(|c| Instant::now().duration_since(c).as_secs() > 30)
        {
            // Lost connection, or too much time has passed since last attempt at 'connect'
            self.set_connection(false)?;
            self.driver.wifi_mut().connect()?;
            info!("Connection lost... scanning");
        }

        Ok(())
    }

    pub fn is_up(&self) -> bool {
        self.driver.is_connected().unwrap_or(false) && self.driver.is_up().unwrap_or(false)
    }
}
