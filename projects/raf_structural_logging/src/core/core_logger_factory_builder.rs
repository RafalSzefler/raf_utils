use std::sync::Arc;

use crate::traits::{LogLevel, StructuralLogHandler, StructuralLoggerFactoryBuilder};

use super::{background_worker::BackgroundWorker, CoreLoggerFactory};

/// Simple core logger factory builder.
#[derive(Default)]
pub struct CoreLoggerFactoryBuilder {
    log_level: LogLevel,
    handlers: Vec<Arc<dyn StructuralLogHandler>>,
}

impl StructuralLoggerFactoryBuilder for CoreLoggerFactoryBuilder {
    type Factory = CoreLoggerFactory;

    
    fn add_handler(&mut self, handler: Arc<dyn StructuralLogHandler>) {
        self.handlers.push(handler);
    }

    fn build(self) -> Self::Factory {
        let worker = BackgroundWorker::new(self.handlers);
        CoreLoggerFactory::new(self.log_level, Arc::new(worker))
    }
    
    fn set_log_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }
}
