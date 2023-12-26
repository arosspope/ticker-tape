
# `ticker-tape`🧾

<p align="center">
  <img src="https://i.imgur.com/sOYVSqa.gif" height="200" />
  <img src="https://i.imgur.com/KkfQT6n.jpg" height="200" />
</p>

This project leverages embedded `rust` support (`std` library) for the `ESP32-S2` microcontroller and `MAX7219` LED dot display to create a Wi-Fi enabled ticker-tape. Upon boot, the device will: 
1. Connect to a local Wi-Fi network (credentials supplied by the user).
2. Advertise the IP address leased by the DHCP server on the ticker display.
2. Start a http server at the aforementioned IP.

Interaction with the `ticker-tape` display over Wi-Fi is possible with simple http `GET/PUT` commands.

## Features

This project **features**:

* Using `std` embedded rust to control an `ESP32-S2` microcontroller. It showcases the logic required to obtain a connection to an access point (AP), start a http server, register multiple http endpoints, and re-establish Wi-Fi connectivity upon link disconnection. All using the `std` crates provided by espressif!

* Driving the `MAX7219` display, converting raw text to displayable patterns on the 8x8 LED grid.

## Environment setup
### Espressif toolchain

To be able to build and deploy this project to your local ESP device, espressif requires the following tools/packages to be available in your local development environment.

Cargo (rust) dependencies:
```sh
$ cargo install cargo-espflash espflash ldproxy
```

System dependencies:
```sh
$ apt install llvm-dev libclang-dev clang libuv-dev libuv1-dev pkgconf python3-venv python-is-python3
```

For further elaboration, please see the [espressif `std` training manual](https://esp-rs.github.io/std-training/02_2_software.html).


### Specify network credentials

Create a file called `cfg.toml` in the root of this repository. The contents need to be substituted with your specific network details.
```sh
$ touch cfg.toml
$ cat cfg.toml
[ticker-tape]
wifi_ssid = "<your wifi ssid>"
wifi_psk = "<your wifi psk>"
```

### Hardware

The hardware required for this project:
* [`ESP32-S2`](https://www.espressif.com/en/products/socs/esp32-s2): ESP32-S2 is a highly integrated, low-power, single-core Wi-Fi Microcontroller SoC, designed to be secure and cost-effective, with a high performance and a rich set of IO capabilities.
* [`MAX7219`](https://core-electronics.com.au/max7219-serial-dot-matrix-display-module.html): Daisy-chainable 8x8 LED dot matrix.

Pin connections from the `MAX7219` to the `ESP32` are as follows:
| MAX7219  | ESP32-S2 |
| :---:    | :---:    |
| Vcc      | 5V       |
| GND      | GND      |
| Din      | GPIO0    |
| CS       | GPIO1    |
| CLK      | GPIO2    |

## Build and deploy

Once the necessary tooling has been installed, building this project should be as simple as running the following from the root of this repository:

```sh
$ cargo build
```

To deploy the built binary to the `ESP32-S2`, connect it to your local machine (with a serial cable) and execute the following:
```sh
$ cargo run
   Compiling ticker-tape v0.1.0 (/home/andrew/projects/ticker-tape-rs)
    Finished dev [optimized + debuginfo] target(s) in 6.37s
     Running `espflash flash --monitor target/xtensa-esp32s2-espidf/debug/ticker-tape`
[2023-12-26T06:24:16Z INFO ] 🚀 A new version of espflash is available: v2.1.0
[2023-12-26T06:24:16Z INFO ] Serial port: '/dev/ttyUSB0'
[2023-12-26T06:24:16Z INFO ] Connecting...
[2023-12-26T06:24:16Z INFO ] Using flash stub
Chip type:         esp32s2 (revision v0.0)
Crystal frequency: 40MHz
Flash size:        4MB
...
I (808) ticker_tape::wifi: Scanning for AP
...
I (55778) wifi:AP's beacon interval = 102400 us, DTIM period = 2
I (56618) esp_netif_handlers: sta ip: 192.168.114.121, mask: 255.255.255.0, gw: 192.168.114.91
I (56618) ticker_tape::display: Set ticker-tape message: "192.168.114.121"
```

## Configuring the `ticker` display over Wi-Fi

Set the ticker display message:
```sh
$ curl -X PUT http://192.168.114.121/message -d "Hello World"
```

Set the scroll speed (ms):
```sh
$ curl -X PUT http://192.168.114.121/speed -d "40"
```

Set the brightness (%):
```sh
$ curl -X PUT http://192.168.114.121/brightness -d "50"
```

Get the current settings from the device:
```sh
~> curl -X GET http://192.168.114.121/ -w "\n"
{"message":"Hello World","speed":40,"brightness":50}
```