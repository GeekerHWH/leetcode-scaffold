use std::{fs, io::Write, path::Path};

use super::Render;

#[derive(Debug)]
pub struct GoLang {
    path: String,
    templates: Templates,
}

#[derive(Debug)]
struct Templates {
    go_mod: String,
    lib: String,
    lib_test: String,
}

impl GoLang {
    pub fn new(path: &str) -> Self {
        GoLang {
            path: path.to_string(),
            templates: Templates {
                go_mod: String::from(include_str!(
                    "../../templates/golang/leetcode-scaffold/go.mod"
                )),
                lib: String::from(include_str!(
                    "../../templates/golang/leetcode-scaffold/lib.go"
                )),
                lib_test: String::from(include_str!(
                    "../../templates/golang/leetcode-scaffold/lib_test.go"
                )),
            },
        }
    }

    fn ensure_dir(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.path)
    }

    fn write_file(&self, filename: &str, content: &str) -> Result<usize, std::io::Error> {
        let file_path = Path::new(&self.path).join(filename);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        file.write(content.as_bytes())
    }
}

impl Render for GoLang {
    fn render_unit_test(&mut self) -> Result<usize, std::io::Error> {
        self.ensure_dir()?;

        let mut total_bytes = 0;
        total_bytes += self.write_file("go.mod", &self.templates.go_mod)?;
        total_bytes += self.write_file("lib.go", &self.templates.lib)?;
        total_bytes += self.write_file("lib_test.go", &self.templates.lib_test)?;

        Ok(total_bytes)
    }
}
