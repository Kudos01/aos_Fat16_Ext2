use std::str;

pub trait Filesystem {
    fn load_info(&mut self, name: &str) -> &mut dyn Filesystem;
    fn print_info(&mut self) -> &mut dyn Filesystem;
    fn find(
        &mut self,
        file_to_find: &str,
        name_of_file: &str,
        delete_flag: bool,
    ) -> &mut dyn Filesystem;
}
