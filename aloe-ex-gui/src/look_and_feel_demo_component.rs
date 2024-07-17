crate::ix!();

pub struct LookAndFeelDemoComponent<'a> {
    base:            Component<'a>,
    rotary_slider:   Slider<'a>,                  // default = { Slider::RotaryHorizontalVerticalDrag, Slider::NoTextBox }
    vertical_slider: Slider<'a>,                  // default = { Slider::LinearVertical, Slider::NoTextBox  }
    bar_slider:      Slider<'a>,                  // default = { Slider::LinearBar, Slider::NoTextBox  }
    inc_dec_slider:  Slider<'a>,                  // default = { Slider::IncDecButtons, Slider::TextBoxBelow  }
    button1:         TextButton<'a>,              // default = { "Hello World!"  }
    button2:         TextButton<'a>,              // default = { "Hello World!"  }
    button3:         TextButton<'a>,              // default = { "Hello World!"  }
    button4:         ToggleButton<'a>,            // default = { "Toggle Me"  }
    radio_buttons:   Vec<Box<TextButton<'a>>>,
}

impl<'a> Default for LookAndFeelDemoComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (rotarySlider);
            rotarySlider.setValue (2.5);

            addAndMakeVisible (verticalSlider);
            verticalSlider.setValue (6.2);

            addAndMakeVisible (barSlider);
            barSlider.setValue (4.5);

            addAndMakeVisible (incDecSlider);
            incDecSlider.setRange (0.0, 10.0, 1.0);
            incDecSlider.setIncDecButtonsMode (Slider::incDecButtonsDraggable_Horizontal);

            addAndMakeVisible (button1);

            addAndMakeVisible (button2);
            button2.setClickingTogglesState (true);
            button2.setToggleState (true, dontSendNotification);

            addAndMakeVisible (button3);

            addAndMakeVisible (button4);
            button4.setToggleState (true, dontSendNotification);

            for (int i = 0; i < 3; ++i)
            {
                auto* b = radioButtons.add (new TextButton ("Button " + String (i + 1)));

                addAndMakeVisible (b);
                b->setRadioGroupId (42);
                b->setClickingTogglesState (true);

                switch (i)
                {
                    case 0:     b->setConnectedEdges (Button::ConnectedOnRight);                            break;
                    case 1:     b->setConnectedEdges (Button::ConnectedOnRight + Button::ConnectedOnLeft);  break;
                    case 2:     b->setConnectedEdges (Button::ConnectedOnLeft);                             break;
                    default:    break;
                }
            }

            radioButtons.getUnchecked (2)->setToggleState (true, dontSendNotification)
        */
    }
}

impl<'a> LookAndFeelDemoComponent<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds().reduced (10);
            auto row = area.removeFromTop (100);

            rotarySlider  .setBounds (row.removeFromLeft (100).reduced (5));
            verticalSlider.setBounds (row.removeFromLeft (100).reduced (5));
            barSlider     .setBounds (row.removeFromLeft (100).reduced (5, 25));
            incDecSlider  .setBounds (row.removeFromLeft (100).reduced (5, 28));

            row = area.removeFromTop (100);
            button1.setBounds (row.removeFromLeft (100).reduced (5));

            auto row2 = row.removeFromTop (row.getHeight() / 2).reduced (0, 10);
            button2.setBounds (row2.removeFromLeft (100).reduced (5, 0));
            button3.setBounds (row2.removeFromLeft (100).reduced (5, 0));
            button4.setBounds (row2.removeFromLeft (100).reduced (5, 0));

            row2 = (row.removeFromTop (row2.getHeight() + 20).reduced (5, 10));

            for (auto* b : radioButtons)
                b->setBounds (row2.removeFromLeft (100));
        */
    }
}
