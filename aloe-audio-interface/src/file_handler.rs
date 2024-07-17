crate::ix!();

pub trait CanHandleFile {

    fn can_handle_file(&mut self, f: &File) -> bool;
}

pub trait GetFileExtensions {

    fn get_file_extensions(&self) -> Vec<String>;
}
