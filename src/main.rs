#![no_std]
#![no_main]

use attiny_hal;
use attiny_hal::prelude::*;
use panic_halt as _;

pub type CoreClock = attiny_hal::clock::MHz1;

#[attiny_hal::entry]
fn main() -> ! {
    //println!("Hello, world!");
    let dp = attiny_hal::Peripherals::take().unwrap();
    // Set clock to divide by 16 (1MHz with osccfg:w:0x01:m)
    // Disable ccp for IOREG using special value
    dp.CPU.ccp.write(|w| unsafe { w.bits(0xD8) } );
    // in 4 cycles, set the clock prescaler
    dp.CLKCTRL.mclkctrlb.write(|w| {
        w.pdiv()._16x();
        w.pen().set_bit()
    });
    let pins = attiny_hal::pins!(dp);
    let mut adc = attiny_hal::Adc::<CoreClock>::new(dp.ADC0, Default::default());
    let (mut spi, _) = attiny_hal::Spi::new(
        dp.SPI0,
        pins.pa3.into_output(),
        pins.pa1.into_output(),
        pins.pa2.into_pull_up_input(),
        pins.pa0.into_output(),
        attiny_hal::spi::Settings::default(),
    );

    let mut led = pins.pa6.into_output();
    led.set_low();
    let adc_pin = pins.pa7.into_analog_input(&mut adc);
    let mut delay = attiny_hal::delay::Delay::<CoreClock>::new();
    delay.delay_ms(1000 as u16);
    let mut i = 0 as u8;
    loop {
        let voltage = adc_pin.analog_read(&mut adc);
        delay.delay_ms(voltage);
        //delay.delay_ms(1000 as u16);
        //led.toggle();
        i += 1;
        //nb::block!(spi.send(i)).void_unwrap();
        nb::block!(spi.send((voltage >> 2) as u8)).void_unwrap();
        led.toggle();
        //dp.CPU.mcucr.write(|w| w.sm().idle());
    }
}
