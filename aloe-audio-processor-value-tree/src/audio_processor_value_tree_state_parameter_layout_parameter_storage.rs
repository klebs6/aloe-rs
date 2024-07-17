crate::ix!();

pub struct AudioProcessorValueTreeStateParameterLayoutParameterStorage<Contents> {
    base:     ParameterStorageBase,
    contents: Box<Contents>,
}

pub trait AudioProcessorValueTreeStateParameterLayoutParameterStorageInterface
{
    fn accept(&mut self, visitor: &dyn AudioProcessorValueTreeStateParameterLayoutVisitor);
}

impl<Contents> AudioProcessorValueTreeStateParameterLayoutParameterStorage<Contents> {

    pub fn new(input: Box<Contents>) -> Self {
    
        todo!();
        /*
        : contents(std::move (input)),

        
        */
    }
    
    pub fn accept(&mut self, visitor: &dyn AudioProcessorValueTreeStateParameterLayoutVisitor)  {
        
        todo!();
        /*
            visitor.visit (std::move (contents));
        */
    }
}

