use std::thread;
use std::time::Instant;

mod cli;
mod devices;
mod format;

use devices::Device;

fn main() {
    let options = match cli::options() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut last = None;

    loop {
        let Device { rx, tx, .. } = devices::get()
            .expect("Get devices data")
            .find(|device| device.interface == options.interface)
            .expect("Missing interface");

        if let Some((instant, last_rx, last_tx)) = last.replace((Instant::now(), rx, tx)) {
            let elapsed = instant.elapsed().as_secs_f64();
            let rx_speed = (rx - last_rx) as f64 / elapsed;
            let tx_speed = (tx - last_tx) as f64 / elapsed;

            if tx_speed > options.tx_threshold {
                println!("↓{}  ↑{}", format::speed(rx_speed), format::speed(tx_speed));
            } else {
                println!("{}", format::speed(rx_speed));
            }
        }

        thread::sleep(options.update_interval);
    }
}
