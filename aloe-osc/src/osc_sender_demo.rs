crate::ix!();

#[no_copy]
#[leak_detector]
pub struct OSCSenderDemo<'a> {
    base:         Component<'a>,
    rotary_knob:  Slider<'a>,
    sender1:      OSCSender,
    sender2:      OSCSender,
    sender_label: Label<'a>, //{ {}, "Sender" };
}

impl<'a> Default for OSCSenderDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (senderLabel);
            senderLabel.attachToComponent (&rotaryKnob, false);

            rotaryKnob.setRange (0.0, 1.0);
            rotaryKnob.setSliderStyle (Slider::RotaryVerticalDrag);
            rotaryKnob.setTextBoxStyle (Slider::TextBoxBelow, true, 150, 25);
            rotaryKnob.setBounds (50, 50, 180, 180);
            addAndMakeVisible (rotaryKnob);
            rotaryKnob.onValueChange = [this]
            {
                // create and send an OSC message with an address and a float value:
                if (! sender1.send ("/aloe/rotaryknob", (float) rotaryKnob.getValue()))
                    showConnectionErrorMessage ("Error: could not send OSC message.");
                if (! sender2.send ("/aloe/rotaryknob", (float) rotaryKnob.getValue()))
                    showConnectionErrorMessage ("Error: could not send OSC message.");
            };

            // specify here where to send OSC messages to: host Url and UDP port number
            if (! sender1.connect ("127.0.0.1", 9001))
                showConnectionErrorMessage ("Error: could not connect to UDP port 9001.");
            if (! sender2.connect ("127.0.0.1", 9002))
                showConnectionErrorMessage ("Error: could not connect to UDP port 9002.")
        */
    }
}

impl<'a> OSCSenderDemo<'a> {
    
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

