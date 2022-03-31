// embedded_hal implementation
use rppal::{spi::{Spi, Bus, SlaveSelect, Mode, Error},hal::Delay};

use hx711_spi::Hx711;
use nb::block;

use std::thread;


fn main() -> Result<(), Error>
{
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0)?;
    let mut hx711 = Hx711::new(spi, Delay::new());

	hx711.reset()?;

    let mut zero_value: f32 = 0.0;
    for i in 0..20 {
        let reading = block!(hx711.read()).unwrap() as f32;
        println!("{:>2}: {}", i, reading);
        zero_value += reading;
    }
    zero_value /= 20.0; //tara

    println!("Tara: {}", zero_value);

    let n = 5;
    loop {
        // let v = block!(hx711.read())?;
        // println!("value = {}", v);


        let mut value: f32 = 0.0;
        for _ in 0..n {
            let reading = block!(hx711.read()).unwrap() as f32;
            value += reading;
        }
        value /= n as f32;
        println!("Read: {} ------ Tara val: {}", value as i32, (value-zero_value) as i32);
        thread::sleep_ms(10);
    }
}
