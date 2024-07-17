crate::ix!();

pub trait DspDemoParameterBaseInterface {
    fn get_component(&mut self) -> *mut Component;
    fn get_preferred_height(&mut self) -> i32;
    fn get_preferred_width(&mut self) -> i32;
}

#[no_copy]
#[leak_detector]
pub struct DSPDemoParameterBase<'a> {
    base: ChangeBroadcaster<'a>,
    name: String,
}

impl<'a> DSPDemoParameterBase<'a> {

    pub fn new(label_name: &String) -> Self {
    
        todo!();
        /*
        : name(labelName),

        
        */
    }
}
