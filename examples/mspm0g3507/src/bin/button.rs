#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_mspm0::gpio::{self, Input, Level, Output, Pull};
use embassy_mspm0::{bind_group, Config};
use embassy_mspm0::peripherals::{GPIOA, GPIOB};
use {defmt_rtt as _, panic_halt as _};

bind_group! {
    /// The GROUP1 handler.
    pub struct Group1 for GROUP1 {
        GPIOA => gpio::InterruptHandler<GPIOA>;
        GPIOB => gpio::InterruptHandler<GPIOB>;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("Hello world!");

    let p = embassy_mspm0::init(Config::default());

    let led1 = p.PA0;
    let s2 = p.PB21;

    let mut led1 = Output::new(led1, Level::Low);

    // FIXME: This allows GPIOA which is wrong.
    //
    // But putting an associated type for the instance the pin
    // belongs to means `AnyPin` cannot be used here.
    let mut s2 = Input::new::<GPIOB>(s2, Group1, Pull::Up);

    // led1 is active low
    led1.set_high();

    loop {
        s2.wait_for_falling_edge().await;

        info!("Switch 2 was pressed");

        led1.toggle();
    }
}
