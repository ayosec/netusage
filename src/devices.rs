use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(PartialEq, Debug)]
pub struct Device {
    pub device: String,
    pub receive_bytes: isize,
    pub transmit_bytes: isize,
}

impl Device {
    pub fn new(device: &str, receive_bytes: isize, transmit_bytes: isize) -> Device {
        Device {
            device: device.into(),
            receive_bytes,
            transmit_bytes,
        }
    }
}

pub fn get() -> std::io::Result<Vec<Device>> {
    parse("/proc/net/dev")
}

fn parse<P: AsRef<Path>>(input: P) -> std::io::Result<Vec<Device>> {
    let devices = BufReader::new(File::open(input)?)
        .lines()
        .skip(2)
        .flat_map(|l| l.ok())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let device = parts.next().map(|d| d.trim_end_matches(':')).unwrap_or("-");
            let receive = parts.next().and_then(|s| s.parse().ok()).unwrap_or(-1);
            let transmit = parts
                .skip(7)
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(-1);

            Device::new(device, receive, transmit)
        })
        .collect();

    Ok(devices)
}

#[test]
fn parse_proc_net_dev() {
    let mut example = parse("src/proc_net_dev.example")
        .expect("Parse example")
        .into_iter();

    assert_eq!(example.next(), Some(Device::new("lo", 100, 100)));
    assert_eq!(example.next(), Some(Device::new("eth0", 758679, 41506)));
    assert_eq!(example.next(), None);
}
