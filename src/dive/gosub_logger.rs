use std::rc::Rc;
use std::sync::{Arc, RwLock};
use log::{SetLoggerError, LevelFilter, Level};

#[derive(Default)]
pub struct GosubLogger {
    logs: Arc<RwLock<Vec<String>>>,
    max_entries: usize,
}

impl GosubLogger {
    pub fn new(max_entries: usize) -> Self {
        Self {
            logs: Arc::new(RwLock::new(vec![])),
            max_entries,
        }
    }

    pub fn init(logger: Rc<GosubLogger>) -> Result<(), SetLoggerError> {
        log::set_max_level(LevelFilter::Trace);
        log::set_boxed_logger(Box::new(logger.clone().as_ref()))
    }

    pub fn inner_log(&self, msg: &str) {
        let mut logs = self.logs.write().unwrap();
        logs.push(msg.into());
        if logs.len() > self.max_entries {
            logs.remove(0);
        }
    }

    pub fn logs(&self) -> Vec<String> {
        self.logs.read().unwrap().clone()
    }
}

impl log::Log for GosubLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &log::Record) {
        let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        if self.enabled(record.metadata()) {
            self.inner_log(format!("{} [{}] {} ", time, record.level(), record.args()).as_str());
        }
    }

    fn flush(&self) {}
}