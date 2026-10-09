use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub(crate) struct Parameters {
    values: HashMap<String, String>,
}

impl Parameters {
    pub(crate) fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub(crate) fn set(&mut self, key: &'static str, value: impl Into<String>) {
        self.values.insert(key.to_owned(), value.into());
    }
}
impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:#?}", self.values)
    }
}
