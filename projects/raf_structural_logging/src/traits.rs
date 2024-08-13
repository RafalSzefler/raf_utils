//! Holds basic structural logging traits.
use std::sync::Arc;

use raf_array::immutable_string::ImmutableString;

use crate::models::LogDataHolder;

/// Represents log level.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

impl Default for LogLevel {
    fn default() -> Self { Self::Debug }
}

/// Rerpresents structure of log.
pub trait StructuralLog {
    fn log_data(&self) -> LogDataHolder;
}

/// Rerpresents logger for structured logs.
pub trait StructuralLogger {
    fn log<T>(&self, log: T) where T : StructuralLog;
}

/// Rerpresents factory for structural loggers.
pub trait StructuralLoggerFactory {
    type Logger : StructuralLogger;

    fn create(&self, name: &ImmutableString) -> Self::Logger;

    fn create_from_str(&self, name: &str) -> Self::Logger {
        let imm = ImmutableString::new(name)
            .expect("StructuralLoggerFactory - create_from_str() fail on new ImmutableString");
        self.create(&imm)
    }
}

/// Rerpresents builder for structural logger factories.
pub trait StructuralLoggerFactoryBuilder {
    type Factory : StructuralLoggerFactory;

    fn add_handler(&mut self, handler: Arc<dyn StructuralLogHandler>);

    fn set_log_level(&mut self, log_level: LogLevel);

    fn build(self) -> Self::Factory;
}

/// Rerpresents handlers for structural logs.
pub trait StructuralLogHandler : Sync + Send {
    fn handle(&self, log: &LogDataHolder);
}
