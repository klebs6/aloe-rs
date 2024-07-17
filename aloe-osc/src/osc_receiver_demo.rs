crate::ix!();

#[no_copy]
#[leak_detector]
pub struct OSCReceiverDemo<'a> {
    base:           Component<'a>,
    base2:          OSCReceiver,
    rotary_knob:    Slider<'a>,
    receiver_label: Label<'a>, // default = { "Receiver"  }
}

impl<'a> OSCReceiverListenerWithOSCAddress<OSCReceiverMessageLoopCallback> 
for OSCReceiverDemo<'a> 
{
    fn osc_message_received(&mut self, message: &OSCMessage)  {
        
        todo!();
        /*
            if (message.size() == 1 && message[0].isFloat32())
                rotaryKnob.setValue (jlimit (0.0f, 10.0f, message[0].getFloat32()));
        */
    }
}

impl<'a> Default for OSCReceiverDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (receiverLabel);
            receiverLabel.attachToComponent (&rotaryKnob, false);

            rotaryKnob.setRange (0.0, 1.0);
            rotaryKnob.setSliderStyle (Slider::RotaryVerticalDrag);
            rotaryKnob.setTextBoxStyle (Slider::TextBoxBelow, true, 150, 25);
            rotaryKnob.setBounds (50, 50, 180, 180);
            rotaryKnob.setInterceptsMouseClicks (false, false);
            addAndMakeVisible (rotaryKnob);

            // specify here on which UDP port number to receive incoming OSC messages
            if (! connect (9001))
                showConnectionErrorMessage ("Error: could not connect to UDP port 9001.");

            // tell the component to listen for OSC messages matching this address:
            addListener (this, "/aloe/rotaryknob")
        */
    }
}

impl<'a> OSCReceiverDemo<'a> {
    
    pub fn show_connection_error_message(&mut self, message_text: &String)  {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                              "Connection error",
                                              messageText,
                                              "OK");
        */
    }
}
