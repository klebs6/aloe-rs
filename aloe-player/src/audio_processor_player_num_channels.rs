crate::ix!();

#[derive(Default)]
pub struct AudioProcessorPlayerNumChannels {
    ins:  i32, // default = 0
    outs: i32, // default = 0
}

impl From<&dyn AudioProcessorBusesLayoutInterface> for AudioProcessorPlayerNumChannels {

    fn from(layout: &dyn AudioProcessorBusesLayoutInterface) -> Self {
    
        todo!();
        /*
        : ins(layout.getNumChannels (true, 0)),
        : outs(layout.getNumChannels (false, 0)),
        */
    }
}

impl AudioProcessorPlayerNumChannels {

    pub fn new(
        num_ins:  i32,
        num_outs: i32) -> Self {
    
        todo!();
        /*
        : ins(numIns),
        : outs(numOuts),

        
        */
    }
    
    pub fn to_layout(&self) -> Box<dyn AudioProcessorBusesLayoutInterface> {
        
        todo!();
        /*
            return { { AudioChannelSet::canonicalChannelSet (ins) },
                         { AudioChannelSet::canonicalChannelSet (outs) } };
        */
    }
}
