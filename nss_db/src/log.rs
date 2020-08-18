use log::{LevelFilter};
use syslog::{BasicLogger, Facility, Formatter3164};

#[allow(unused_must_use)]
pub fn setup_log<S: Into<String>>(name: S) {
    let formatter = Formatter3164 {
        facility: Facility::LOG_AUTHPRIV,
        hostname: None,
        process: name.into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Info));
}
