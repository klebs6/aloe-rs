crate::ix!();

#[no_copy]
#[leak_detector]
pub struct RadioButtonsGroupComponent<'a> {
    base:                Component<'a>,
    radio_buttons:       [ToggleButton<'a>; NUM_RADIO_BUTTONS],
    radio_buttons_group: Box<RadioButtonsGroupComponent<'a>>,
    text_button:         TextButton<'a>,  //{ "Press me!" };
    shape_button:        ShapeButton<'a>, //{ "Pressable Aloe Logo", Colours::darkorange, Colours::darkorange.brighter (0.5f), Colours::darkorange.brighter (0.75f) };
}

const NUM_RADIO_BUTTONS: usize = 3;

impl<'a> Default for RadioButtonsGroupComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            int index = 1;
                    for (auto& b : radioButtons)
                    {
                        b.setRadioGroupId (1);
                        b.setButtonText ("Button " + String (index++));
                        addAndMakeVisible (b);
                    }

                    radioButtons[(size_t) Random::getSystemRandom().nextInt (NUM_RADIO_BUTTONS)].setToggleState (true, dontSendNotification);

                    setTitle ("Radio Buttons Group");
                    setFocusContainerType (FocusContainerType::focusContainer)
        */
    }
}

impl<'a> Resized for RadioButtonsGroupComponent<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();
                    const auto height = bounds.getHeight() / NUM_RADIO_BUTTONS;

                    for (auto& b : radioButtons)
                        b.setBounds (bounds.removeFromTop (height).reduced (2));
        */
    }
}
