crate::ix!();

pub struct OSCReceiverImplCallbackMessage {
    base: Message,

    /**
      | the payload of the message. Can be either
      | an OSCMessage or an OSCBundle.
      |
      */
    content: OSCBundleElement,
}

impl OSCReceiverImplCallbackMessage {

    pub fn new(osc_element: OSCBundleElement) -> Self {
    
        todo!();
        /*
        : content(oscElement),

        
        */
    }
}
