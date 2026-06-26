use esp_hal::delay::Delay;
use esp_hal::gpio::Output;
use esp_hal::spi::master::Spi;
use esp_hal::Blocking;

use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use mipidsi::{
    Builder,
    models::ST7735s,
    interface::SpiInterface,
    options::{Orientation, Rotation},
};

pub fn init_display<'a>(
    spi: Spi<'a, Blocking>,
    cs: Output<'a>,
    dc: Output<'a>,
    rst: Output<'a>,
    delay: &'a mut Delay,
    buffer: &'a mut [u8],
) -> mipidsi::Display<
    SpiInterface<'a,
        ExclusiveDevice<Spi<'a, Blocking>, Output<'a>, NoDelay>,
        Output<'a>,
    >,
    ST7735s,
    Output<'a>,
>{
    let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let di = SpiInterface::new(spi_dev, dc, buffer);

    Builder::new(ST7735s, di)
        .reset_pin(rst)
        .display_size(128, 160)
        .display_offset(0, 0)
        .orientation(
    Orientation::new()
        .rotate(Rotation::Deg90)
)
        .init(delay)
        .unwrap()
}