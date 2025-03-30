use std::{fs, io::Write};

use super::Render;

#[derive(Debug)]
pub struct RustLang {
    path: String,
    unit_test_template: String,
}

impl RustLang {
    pub fn new(path: &str) -> Self {
        RustLang {
            path: path.to_string(),
            unit_test_template: String::from(include_str!("../../templates/rust.rs")),
        }
    }
}

impl Render for RustLang {
    fn render_unit_test(&mut self) -> Result<usize, std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        file.write(self.unit_test_template.as_bytes())
    }
}
