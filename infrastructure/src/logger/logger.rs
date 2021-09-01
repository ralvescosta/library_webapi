use application::interfaces::i_logger::ILogger;
use log::{debug, error, info, trace, warn};
pub struct Logger;

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }
}

impl ILogger for Logger {
    fn trace(&self, msg: String) {
        trace!("{}", msg)
    }

    fn debug(&self, msg: String) {
        debug!("{}", msg)
    }

    fn info(&self, msg: String) {
        info!("{}", msg)
    }

    fn warn(&self, msg: String) {
        warn!("{}", msg)
    }

    fn error(&self, msg: String) {
        error!("{}", msg)
    }
}
