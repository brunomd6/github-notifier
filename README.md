# Github Notifier

## Hardware
- ESP32 DEVKITV1
- ST7735 1.77 128x160 RGB Display


## Pinout
| ST7735 | ESP32 DEVKITV1      |
|--------|---------------------|
| GND    | GND                 |
| VCC    | 3V3                 |
| SCL    | GPIO_18 (VSPI_CLK)  |
| SDA    | GPIO_23 (VSPI_MOSI) |
| RST    | GPIO_25             |
| DC     | GPIO_26             | 
| CS     | GPIO_5 (VSPI_CS)    |
| BL     | GPIO_27             |


## Build Docker Image

```
docker build -t esp32-embassy .
```

## Run Docker

```
docker run --rm -it -v ${PWD}:/workspace -w /workspace esp32-test
```


## How to Compile

```
source ~/export-esp.sh
cd /workspace/github-notifier
cargo build
```

```
docker run --rm -it -v ${PWD}:/workspace -w /workspace esp32-test bash -c "source /root/export-esp.sh && cd /workspace/github_notifier && cargo build"
```

## How to Flash

```
espflash flash target\xtensa-esp32-none-elf\debug\github_notifier --chip esp32 --port COM4 --baud 921600
```

## Do Everything at Once

```
docker run --rm -it -v ${PWD}:/workspace -w /workspace esp32-test bash -c "source /root/export-esp.sh && cd /workspace/github_notifier && cargo build"; espflash flash github_notifier\target\xtensa-esp32-none-elf\debug\github_notifier --chip esp32 --port COM10 --baud 921600
```

## Format

```
docker run --rm -it -v ${PWD}:/workspace -w /workspace/github_notifier esp32-test bash -c "cargo fmt"
```
