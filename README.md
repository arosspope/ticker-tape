
# Foo

<p align="centre">
  <img src="https://i.imgur.com/V4uO5r2.gif" height="300" />
  <img src="https://i.imgur.com/YFpuC69.gif" height="300" />
  <img src="https://i.imgur.com/KkfQT6n.jpg" height="300" />
</p>

## Setup

```sh
$ cargo build
```
### Network credentials

WPA-X(?)

### Setting hostname of device

## Running

### deploy

```sh
$ cargo run
```

### interact

```sh
~> curl -X GET http://192.168.1.106 -w "\n"
{"message":"192.168.1.106 ","speed":70,"brightness":204}

~> curl -X PUT http://192.168.1.106/message -d "${date +'%a %d %b %Y'}"


~> curl -X PUT http://192.168.1.106/speed -d "40"
```