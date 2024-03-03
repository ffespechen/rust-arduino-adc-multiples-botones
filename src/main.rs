#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());


    let a1 = pins.a1.into_analog_input(&mut adc);

    loop {
        ufmt::uwriteln!(&mut serial, "{}", a1.analog_read(&mut adc)).unwrap();
        arduino_hal::delay_ms(500);
    }
}
