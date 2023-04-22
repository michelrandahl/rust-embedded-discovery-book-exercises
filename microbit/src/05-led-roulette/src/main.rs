#![deny(unsafe_code)]
// these two attributes are used for embedded programming
// (std would assume access to an underlying operating system)
#![no_main]
#![no_std]

// required to add the type constraint in function signature
#![feature(generic_const_exprs)]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer
};

const PIXELS: [(usize, usize); 25] = [
    (4,0), (3,0), (2,0), (1,0),
    (0,0), (0,1), (0,2), (0,3), (0,4),
    (1,4), (2,4), (3,4), (4,4),
    (4,3), (4,2), (4,1),
    (3,1), (2,1), (1,1),
    (1,2), (1,3),
    (2,3), (3,3),
    (3,2), (2,2)
];

fn reverse_array<const N: usize>(arr: &[(usize, usize); N]) -> [(usize, usize); N] {
    let mut reversed: [(usize, usize); N] = [(0, 0); N];
    for (index, value) in arr.iter().enumerate() {
        reversed[arr.len() - index - 1] = *value;
    }
    reversed
}

fn concatenate_arrays2<const N: usize, const M: usize>(
    arr1: &[(usize, usize); N],
    arr2: &[(usize, usize); M],
) -> [(usize, usize); N + M] {
    let mut concatenated: [(usize, usize); N + M] = [(0, 0); N + M];

    for i in 0..N {
        concatenated[i] = arr1[i];
    }
    for i in 0..M {
        concatenated[N + i] = arr2[i];
    }
    concatenated
}
// `fn() -> !` means that the function never terminates
#[entry]
fn main() -> ! {
    rtt_init_print!();

    let reversed_pixels = reverse_array(&PIXELS);
    let new_pixels = concatenate_arrays2(&PIXELS, &reversed_pixels);

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut last_led = (0,0);

    let mut delay_time = 100;
    let mut count_up = true;

    loop {
        for current_led in new_pixels.iter() {
            leds[last_led.0][last_led.1] = 0;
            leds[current_led.0][current_led.1] = 1;
            display.show(&mut timer, leds, delay_time);
            last_led = *current_led;

            if count_up {
                if delay_time < 100 {
                    delay_time += 1;
                } else {
                    delay_time -= 1;
                    count_up = false;
                }
            } else {
                if delay_time > 0 {
                    delay_time -= 1;
                } else {
                    delay_time += 1;
                    count_up = true;
                }
            }
        }
    }
}
// $ cargo embed --features v2 --target thumbv7em-none-eabihf
// $ gdb target/thumbv7em-none-eabihf/debug/led-roulette
// inside gdb: source ./src/05-led-roulette/gdb.txt
