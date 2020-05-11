pub fn speed(speed: f64) -> String {
    let items = [(1024 * 1024 * 1024, "G"), (1024 * 1024, "M"), (1024, "K")];

    match items.iter().find(|(d, _)| speed > *d as f64) {
        Some((d, suffix)) => format!("{:.1} {}B/s", speed / *d as f64, suffix),
        None => format!("{:.0} B/s", speed),
    }
}

#[test]
fn format_speed() {
    assert_eq!(speed(3250585.6), String::from("3.1 MB/s"));
    assert_eq!(speed(9019431321.6), String::from("8.4 GB/s"));
    assert_eq!(speed(5000.0), String::from("4.9 KB/s"));
    assert_eq!(speed(1.0), String::from("1 B/s"));
}
