pub mod go;
pub mod rust;
pub trait Render {
    fn render_unit_test(&mut self) -> Result<usize, std::io::Error>;
}
