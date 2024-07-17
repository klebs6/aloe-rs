crate::ix!();

pub struct AudioSettingsGroup<'a> {
    base:                 Component<'a>,
    title_label:          Label<'a>, //{ {}, "Audio" };
    device_selector_comp: AudioDeviceSelectorComponent<'a>,
}

impl<'a> Default for AudioSettingsGroup<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : deviceSelectorComp (getSharedAudioDeviceManager(), 0, 256, 0, 256, true, true, true, false)

                addAndMakeVisible (titleLabel);
                titleLabel.setFont (titleLabelFontHeight);

                addAndMakeVisible (deviceSelectorComp);
                deviceSelectorComp.setItemHeight (itemHeight);

                setFocusContainerType (FocusContainerType::focusContainer);
                setTitle ("Audio Settings")
        */
    }
}

impl<'a> AudioSettingsGroup<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

                titleLabel.setBounds (bounds.removeFromTop (itemHeight));
                bounds.removeFromTop (itemSpacing);

                deviceSelectorComp.setBounds (bounds);
        */
    }
}

