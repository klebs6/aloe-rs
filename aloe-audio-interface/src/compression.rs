crate::ix!();

pub trait IsCompressed {

    fn is_compressed(&mut self) -> bool;
}
