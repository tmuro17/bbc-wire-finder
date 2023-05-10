#![deny(unsafe_code, clippy::pedantic)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use libm::{round, sqrt};
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate, Measurement};
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

type Lights = [[u8; 5]; 5];

const OFF: Lights = [[0; 5]; 5];

const CENTER: Lights = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

const INNER_RING: Lights = [
    [0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0],
];

const OUTER_RING: Lights = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
];

enum SignalLevel {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

impl From<Measurement> for SignalLevel {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    // No good way to get the f64 that we know can be a u8 to an actual u8 without a potentially (but not in our case) lossy cast.
    fn from(data: Measurement) -> Self {
        let x = f64::from(data.x);
        let y = f64::from(data.y);
        let z = f64::from(data.z);

        let magnitude: f64 = sqrt(x * x + y * y + z * z);
        let scaled = (magnitude - 20_000.0) / 150_000.0;
        let clamped = scaled.clamp(0.0, 5.0);
        let level = round(clamped) as u8;

        match level {
            1 => SignalLevel::Level1,
            2 => SignalLevel::Level2,
            3 => SignalLevel::Level3,
            4 => SignalLevel::Level4,
            5 => SignalLevel::Level5,
            _ => SignalLevel::Level0,
        }
    }
}

impl From<SignalLevel> for Lights {
    fn from(level: SignalLevel) -> Self {
        match level {
            SignalLevel::Level0 => OFF,
            SignalLevel::Level1 => OUTER_RING,
            SignalLevel::Level2 => INNER_RING,
            SignalLevel::Level3 => CENTER,
            SignalLevel::Level4 => combine(CENTER, INNER_RING),
            SignalLevel::Level5 => combine(combine(CENTER, INNER_RING), OUTER_RING),
        }
    }
}

fn combine(x: Lights, y: Lights) -> Lights {
    let mut result = [[0; 5]; 5];
    x.iter()
        .zip(y.iter())
        .enumerate()
        .for_each(|(row, (x, y))| {
            x.iter()
                .zip(y.iter())
                .enumerate()
                .for_each(|(col, (x, y))| {
                    result[row][col] = x | y;
                });
        });

    result
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Starting!");
    let board = Board::take().unwrap();
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data {}
        let data = sensor.mag_data().unwrap();
        let level = SignalLevel::from(data);
        display.clear();

        let lights: Lights = Lights::from(level);

        display.show(&mut timer, lights, 100);
    }
}
