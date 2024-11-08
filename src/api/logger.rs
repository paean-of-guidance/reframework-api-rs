use log::{Level, Log, Metadata, Record};

use crate::RefAPI;

pub struct RefLogger {
    prefix: String,
}

impl Log for RefLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(api) = RefAPI::instance() {
                match record.level() {
                    Level::Error => api.log().error(&format!("[{}] {}", self.prefix, record.args())),
                    Level::Warn => api.log().warn(&format!("[{}] {}", self.prefix, record.args())),
                    Level::Info => api.log().info(&format!("[{}] {}", self.prefix, record.args())),
                    Level::Debug => api.log().info(&format!("[{}] DEBUG {}", self.prefix, record.args())),
                    Level::Trace => api.log().info(&format!("[{}] TRACE {}", self.prefix, record.args())),
                }
            }
        }
    }

    fn flush(&self) {}
}

impl RefLogger {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    #[allow(unused)]
    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
    }
}
