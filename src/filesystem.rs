use std::str;

pub trait Filesystem {
    fn load_info(&mut self, name: &str) -> &mut dyn Filesystem;
    fn print_info(&mut self) -> &mut dyn Filesystem;
}
