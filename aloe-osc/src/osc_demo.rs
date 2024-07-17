crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/OSCDemo.h]

#[no_copy]
#[leak_detector]
pub struct OSCDemo<'a> {
    base:     Component<'a>,
    monitor:  OSCMonitorDemo<'a>,
    receiver: OSCReceiverDemo<'a>,
    sender:   OSCSenderDemo<'a>,
}

impl<'a> Default for OSCDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (monitor);
            addAndMakeVisible (receiver);
            addAndMakeVisible (sender);

            setSize (700, 400)
        */
    }
}

impl<'a> OSCDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

            auto lowerBounds = bounds.removeFromBottom (getHeight() / 2);
            auto halfBounds  = bounds.removeFromRight  (getWidth()  / 2);

            sender  .setBounds (bounds);
            receiver.setBounds (halfBounds);
            monitor .setBounds (lowerBounds.removeFromTop (getHeight() / 2));
        */
    }
}
