crate::ix!();

pub trait FileChooserPimpl
{
    fn launch(&mut self);
    fn run_modally(&mut self);
}
