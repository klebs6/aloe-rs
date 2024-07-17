crate::ix!();

pub struct OSCReceiverPimplCallbackMessage {
    base: Message,

    /**
      | the payload of the message. Can be either
      | an OSCMessage or an OSCBundle.
      |
      */
    content: OSCBundleElement,
}

impl OSCReceiverPimplCallbackMessage {

    pub fn new(osc_element: OSCBundleElement) -> Self {
    
        todo!();
        /*
        : content(oscElement),

        
        */
    }
}
