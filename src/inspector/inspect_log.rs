#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use bevy::prelude::*;

#[derive(Clone, Default, Resource)]
pub struct Logs {
    logs: Arc<RwLock<HashMap<LogItem, u64>>>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct LogItem {
    pub level_log: log::Level,
    pub module: String,
    pub file: String,
    pub line: String,
    pub details: String,
}

impl<'a> From<&log::Record<'a>> for LogItem {
    fn from(v: &log::Record) -> Self {
        Self {
            level_log: v.level(),
            module: v.module_path().unwrap_or_default().to_string(),
            file: v.file().unwrap_or_default().to_string(),
            line: v.line().unwrap_or_default().to_string(),
            details: v.args().to_string(),
        }
    }
}

impl Logs {
    pub fn clear(&self) {
        let mut logs = self.logs.write().unwrap();
        logs.clear();
        drop(logs);
    }

    pub fn len(&self) -> usize {
        let logs = self.logs.read().unwrap();
        let len = logs.len();
        drop(logs);
        len
    }

    pub fn get_logs(&self) -> HashMap<LogItem, u64> {
        let logs = self.logs.read().unwrap();
        let v = logs.clone();
        drop(logs);
        v
    }
}

impl log::Log for Logs {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        if !record
            .module_path()
            .is_some_and(|m| m.starts_with(env!("CARGO_PKG_NAME")))
        {
            return;
        }
        let mut logs = self.logs.write().unwrap();
        let item = LogItem::from(record);
        (*logs)
            .entry(item)
            .and_modify(|c| {
                if *c <= 99 {
                    *c += 1;
                }
            })
            .or_insert(0);
        drop(logs);
    }

    fn flush(&self) {}
}
