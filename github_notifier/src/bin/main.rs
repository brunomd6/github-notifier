#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::delay::Delay;

use esp_hal::{
    spi::{
        master::{Config, Spi},
        Mode,
    },
    time::Rate,
};
    
use embedded_hal_bus::spi::ExclusiveDevice;
use mipidsi::{
    Builder,
    models::ST7735s,
    interface::SpiInterface,
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    text::{Text, Baseline},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
};

use static_cell::StaticCell;

static BUFFER: StaticCell<[u8; 128]> = StaticCell::new();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32 -o alloc -o esp32-wroom-32 -o unstable-hal -o embassy

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut delay = Delay::new();

    // The following pins are used to bootstrap the chip. They are available
    // for use, but check the datasheet of the module for more information on them.
    // - GPIO0
    // - GPIO2
    // - GPIO5
    // - GPIO12
    // - GPIO15
    // These GPIO pins are in use by some feature of the module and should not be used.
    let _ = peripherals.GPIO6;
    let _ = peripherals.GPIO7;
    let _ = peripherals.GPIO8;
    let _ = peripherals.GPIO9;
    let _ = peripherals.GPIO10;
    let _ = peripherals.GPIO11;
    let _ = peripherals.GPIO16;
    let _ = peripherals.GPIO20;

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    // TODO: Spawn some tasks
    let _ = spawner;

    esp_println::println!("something!");

    let mut backlight = Output::new(peripherals.GPIO27, Level::High, OutputConfig::default());
    backlight.set_low();

    
    // SPI pins
    let sclk = peripherals.GPIO18;
    let mosi = peripherals.GPIO23;
    let cs   = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());

    // control pins
    let dc  = Output::new(peripherals.GPIO26, Level::Low, OutputConfig::default());
    let mut rst = Output::new(peripherals.GPIO25, Level::High, OutputConfig::default());

    Timer::after(Duration::from_millis(50)).await;
    rst.set_low();
    Timer::after(Duration::from_millis(10)).await;
    rst.set_high();
    Timer::after(Duration::from_millis(120)).await;

    // SPI
    let spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_mhz(10))
            .with_mode(Mode::_0),
    )
    .expect("SPI init failed")
    .with_sck(sclk)
    .with_mosi(mosi)
    .with_miso(peripherals.GPIO19); // can be ignored for display

    let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let buffer = BUFFER.init([0u8; 128]);

    let di = SpiInterface::new(
        spi_dev,
        dc,
        buffer,
    );

    let mut display = Builder::new(ST7735s, di)
        .reset_pin(rst)
        .display_size(128, 160)
        .display_offset(2, 1)
        .init(&mut delay)
        .unwrap();

    display.clear(Rgb565::BLACK).unwrap();

    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    Text::with_baseline(
    "Hello ESP32",
    Point::new(10, 20),
    style,
    Baseline::Top,
)
.draw(&mut display)
.unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
