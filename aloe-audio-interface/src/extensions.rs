crate::ix!();

pub trait GetExtensions {

    fn get_extensions(&self, visitor: &mut dyn ExtensionsVisitorInterface);
}
