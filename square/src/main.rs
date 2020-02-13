use std::env;
use std::str::FromStr;
use std::thread;
use std::time;

fn main() {
    let args: Vec<String> = env::args().collect();
    let freq = f32::from_str(&args[1]).unwrap();
    let r = f32::from_str(&args[2]).unwrap();
    let vol = f32::from_str(&args[3]).unwrap();

    let wl = (20_000_f32 / freq.round()).round();
    let onl = (wl * r).round();

    let mut wave = String::new();
    for w in 0..(freq * 2.).round() as i32 {
        for i in 0..wl as i32 {
            if i < onl as i32 {
                wave.push_str(&format!("{}", vol));
            } else {
                wave.push_str("0");
            }
            wave.push_str(" ");
        }
    }
    println!("{}", wave);

    loop {
        let mut wave = String::new();
        for w in 0..(freq / 5_f32).round() as i32 {
            for i in 0..wl as i32 {
                if i < onl as i32 {
                    wave.push_str(&format!("{}", vol));
                } else {
                    wave.push_str("0");
                }
                wave.push_str(" ");
            }
        }
        println!("{}", wave);
        thread::sleep(time::Duration::from_secs_f32(1_f32 / 8_f32));
    }
}
