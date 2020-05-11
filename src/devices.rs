use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(PartialEq, Debug)]
pub struct Device {
    pub interface: String,
    pub rx: isize,
    pub tx: isize,
}

impl Device {
    pub fn new(interface: &str, rx: isize, tx: isize) -> Device {
        Device {
            interface: interface.into(),
            rx,
            tx,
        }
    }
}

pub fn get() -> std::io::Result<impl Iterator<Item = Device>> {
    parse("/proc/net/dev")
}

fn parse<P: AsRef<Path>>(input: P) -> std::io::Result<impl Iterator<Item = Device>> {
    let devices = BufReader::new(File::open(input)?)
        .lines()
        .skip(2)
        .flat_map(|l| l.ok())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let interface = parts.next().map(|d| d.trim_end_matches(':')).unwrap_or("-");
            let receive = parts.next().and_then(|s| s.parse().ok()).unwrap_or(-1);
            let transmit = parts
                .skip(7)
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(-1);

            Device::new(interface, receive, transmit)
        });

    Ok(devices)
}

#[test]
fn parse_proc_net_dev() {
    let mut example = parse("src/proc_net_dev.example").expect("Parse example");

    assert_eq!(example.next(), Some(Device::new("lo", 100, 100)));
    assert_eq!(example.next(), Some(Device::new("eth0", 758679, 41506)));
    assert_eq!(example.next(), None);
}
