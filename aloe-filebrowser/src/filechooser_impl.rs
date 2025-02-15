crate::ix!();

pub trait FileChooserImpl
{
    fn launch(&mut self);
    fn run_modally(&mut self);
}
