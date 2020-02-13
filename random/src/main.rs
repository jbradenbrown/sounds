extern crate rand;

use rand::distributions::uniform::Uniform;
use rand::prelude::*;
use std::env;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let center = f32::from_str(&args[1]).unwrap();
    let movement = f32::from_str(&args[2]).unwrap();

    let mut rng = rand::thread_rng();

    let sampler = rng.sample_iter(Uniform::new(-movement, movement));
    let samples: String = sampler
        .take(100_000)
        .scan(center, |state, n| {
            *state = (n + 1.0) * ((*state) / ((2.0 - (center / *state)).abs()));
            Some(*state / (1.0 + state.powf(2.0)))
        })
        .fold("".to_string(), |a, n| format!("{} {:.3}", a, n));
    println!("{}", samples);

    loop {
        let sampler = rng.sample_iter(Uniform::new(-movement, movement));
        let samples: String = sampler
            .take(20_000)
            .scan(center, |state, n| {
                *state = (n + 1.0) * ((*state) / ((2.0 - (center / *state)).abs()));
                Some(*state / (1.0 + state.powf(2.0)))
            })
            .fold("".to_string(), |a, n| format!("{} {:.3}", a, n));
        println!("{}", samples);
    }
}
