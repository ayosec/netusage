pub fn speed(speed: f64) -> String {
    let items = [
        (1024 * 1024 * 1024, "G", 1),
        (1024 * 1024, "M", 1),
        (1024, "K", 0),
    ];

    match items.iter().find(|(d, _, _)| speed > *d as f64) {
        Some((d, suffix, prec)) => format!("{:.2$} {}B/s", speed / *d as f64, suffix, prec),
        None if speed > 0.0 => String::from("<1 KB/s"),
        None => String::from("0 KB/s"),
    }
}

#[test]
fn format_speed() {
    assert_eq!(speed(3250585.6), String::from("3.1 MB/s"));
    assert_eq!(speed(9019431321.6), String::from("8.4 GB/s"));
    assert_eq!(speed(5000.0), String::from("5 KB/s"));
    assert_eq!(speed(100.0), String::from("<1 KB/s"));
    assert_eq!(speed(0.0), String::from("0 KB/s"));
}
