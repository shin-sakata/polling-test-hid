use std::time::{Duration, Instant};

use anyhow::Result;
use hidapi::HidApi;

const VID: u16 = 0x054C;
const PID: u16 = 0x0DF2;

fn main() -> Result<()> {
    let api = HidApi::new()?;
    let mut buf = [0u8; 10];
    loop {
        match api.open(VID, PID) {
            Ok(device) => {
                println!("DualSense Edge connected");

                let mut counter = 0;
                let mut last_log_time = Instant::now();

                loop {
                    match device.read_timeout(&mut buf, 10000) {
                        Ok(_) => {
                            counter += 1;
                            let elapsed = last_log_time.elapsed();
                            if elapsed >= Duration::from_secs(1) {
                                let packets_per_sec = counter as f64 / elapsed.as_secs_f64();
                                println!("Packets per second: {:.2}", packets_per_sec);
                                counter = 0;
                                last_log_time = Instant::now();
                            }
                        }
                        Err(err) => {
                            println!("Failed to read from DualSense Edge: {}", err);
                            break;
                        }
                    }
                }
                println!("DualSense Edge disconnected");
            }
            Err(err) => {
                println!("Failed to open DualSense Edge: {}", err);
                println!("Retrying in 1 second...");
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
