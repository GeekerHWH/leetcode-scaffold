use std::{fs::File, io::Write, string};

use super::Render;

#[derive(Debug)]
pub struct RustLang {
    file: File,
    unit_test_template: String,
}

impl RustLang {
    pub fn new(path: &str) -> Self {
        RustLang {
            file: match File::create_new(path) {
                Ok(f) => f,
                Err(_) => File::open(path).expect("Open file Error"),
            },
            unit_test_template: String::from("fn main(){}"),
        }
    }
}

impl Render for RustLang {
    fn render_unit_test(&mut self) -> Result<usize, std::io::Error> {
        self.file.write(self.unit_test_template.as_bytes())
    }
}
