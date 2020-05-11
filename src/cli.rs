use std::time::Duration;

const HELP: &str = "netusage: print network usage

-i IFACE    Interface to monitor
-u SECONDS  Interval to print the data
-t RATE     Threshold to show transmit bytes/s
";

pub struct Options {
    pub interface: String,
    pub update_interval: Duration,
    pub tx_threshold: f64,
}

pub fn options() -> Result<Options, &'static str> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        return Err(HELP);
    }

    let interface = match args.opt_value_from_str("-i") {
        Ok(Some(i)) => i,
        _ => return Err("Missing -i option"),
    };

    let update_interval = match args.opt_value_from_str("-u") {
        Ok(None) => Duration::from_secs(2),
        Ok(Some(u)) => Duration::from_secs(u),
        _ => return Err("Invalid value for -u"),
    };

    let tx_threshold = match args.opt_value_from_str("-t") {
        Ok(None) => 102400.0,
        Ok(Some(u)) => u,
        _ => return Err("Invalid value for -t"),
    };

    Ok(Options {
        interface,
        update_interval,
        tx_threshold,
    })
}
