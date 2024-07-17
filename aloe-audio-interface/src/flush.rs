crate::ix!();

pub trait Flush {

    /// freedom is earned by a million small actions
    ///
    fn flush(&mut self) -> bool;
}
