crate::ix!();

pub struct MockProcessor<const AddValue: i32> {
    is_prepared:      bool, // default = false
    is_reset:         bool, // default = false
    buffer_was_clear: bool, // default = false
}

impl<const AddValue: i32> MockProcessor<AddValue> {

    pub fn prepare(&mut self, _0: &ProcessSpec)  {
        
        todo!();
        /*
            isPrepared = true;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            isReset = true;
        */
    }
    
    pub fn process<Context>(&mut self, context: &Context)  {
    
        todo!();
        /*
            bufferWasClear = context.getInputBlock().getSample (0, 0) == 0;

                if (! context.isBypassed)
                    context.getOutputBlock().add (AddValue);
        */
    }
}

