extern crate cpal;

use std::iter::successors;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};

fn main() {
    let ev = setup();
    let tx = stream(ev);

    loop {
        let mut v_str = String::new();
        std::io::stdin().read_line(&mut v_str);
        let vs = v_str.trim().split(' ').map(|e| f32::from_str(e));
        for v in vs {
            tx.send(v.expect("Couldn't parse float!"));
        }
    }
}

fn setup() -> cpal::EventLoop {
    let host = cpal::default_host();
    let event_loop = host.event_loop();

    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_formats_range = device
        .supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range
        .next()
        .expect("no supported format?!")
        .with_max_sample_rate();

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop
        .play_stream(stream_id)
        .expect("failed to play_stream");

    event_loop
}

fn stream(event_loop: cpal::EventLoop) -> std::sync::mpsc::Sender<f32> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        event_loop.run(move |stream_id, stream_result| {
            let stream_data = match stream_result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                    return;
                }
                _ => return,
            };
            match stream_data {
                StreamData::Output {
                    buffer: UnknownTypeOutputBuffer::U16(mut buffer),
                } => {
                    for elem in buffer.iter_mut() {
                        *elem = u16::max_value() / 2;
                    }
                }
                StreamData::Output {
                    buffer: UnknownTypeOutputBuffer::I16(mut buffer),
                } => {
                    for elem in buffer.iter_mut() {
                        *elem = 0;
                    }
                }
                StreamData::Output {
                    buffer: UnknownTypeOutputBuffer::F32(mut buffer),
                } => {
                    for elem in buffer.iter_mut() {
                        let res = rx.recv();
                        if let Ok(v) = res {
                            *elem = v;
                        } else {
                            println!("AHHHHH!");
                        }
                    }
                }
                _ => (),
            }
        })
    });
    return tx;
}

fn sine(wavelength: f32) -> Box<dyn Iterator<Item = f32>> {
    let counter = successors(Some(0_f32), |p| Some(p + 1_f32));
    Box::new(counter.map(move |e| e / wavelength).map(|e| e.sin()))
}
