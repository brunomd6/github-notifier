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
docker run --rm -it -v ${PWD}:/workspace -w /workspace esp32-test bash -c "source /root/export-esp.sh && cd /workspace/github_notifier && cargo build && espflash save-image --chip esp32 target/xtensa-esp32-none-elf/debug/github_notifier github_notifier.bin"
```


## How to Flash

```
espflash flash -p COM4 target\xtensa-esp32-none-elf\debug\github-notifier
```

```
esptool -p COM4 -b 115200 --chip esp32 --before default_reset --after hard_reset write_flash 0x0 github_notifier/github_notifier.bin
```
