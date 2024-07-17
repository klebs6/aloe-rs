crate::ix!();

#[no_copy]
#[leak_detector]
pub struct StandalonePluginHolderSettingsComponent<'a> {
    base:               Component<'a>,
    owner:              &'a mut StandalonePluginHolder<'a>,
    device_selector:    AudioDeviceSelectorComponent<'a>,
    should_mute_label:  Label<'a>,
    should_mute_button: ToggleButton<'a>,
    is_resizing:        bool, // default = false
}

impl<'a> StandalonePluginHolderSettingsComponent<'a> {

    pub fn new(
        plugin_holder:             &mut StandalonePluginHolder,
        device_manager_to_use:     &mut AudioDeviceManager,
        max_audio_input_channels:  i32,
        max_audio_output_channels: i32) -> Self {
    
        todo!();
        /*


            : owner (pluginHolder),
                  deviceSelector (deviceManagerToUse,
                                  0, maxAudioInputChannels,
                                  0, maxAudioOutputChannels,
                                  true,
                                  (pluginHolder.processor.get() != nullptr && pluginHolder.processor->producesMidi()),
                                  true, false),
                  shouldMuteLabel  ("Feedback Loop:", "Feedback Loop:"),
                  shouldMuteButton ("Mute audio input")

                setOpaque (true);

                shouldMuteButton.setClickingTogglesState (true);
                shouldMuteButton.getToggleStateValue().referTo (owner.shouldMuteInput);

                addAndMakeVisible (deviceSelector);

                if (owner.getProcessorHasPotentialFeedbackLoop())
                {
                    addAndMakeVisible (shouldMuteButton);
                    addAndMakeVisible (shouldMuteLabel);

                    shouldMuteLabel.attachToComponent (&shouldMuteButton, true);
                }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            const ScopedValueSetter<bool> scope (isResizing, true);

                auto r = getLocalBounds();

                if (owner.getProcessorHasPotentialFeedbackLoop())
                {
                    auto itemHeight = deviceSelector.getItemHeight();
                    auto extra = r.removeFromTop (itemHeight);

                    auto seperatorHeight = (itemHeight >> 1);
                    shouldMuteButton.setBounds (Rectangle<int> (extra.proportionOfWidth (0.35f), seperatorHeight,
                                                                extra.proportionOfWidth (0.60f), deviceSelector.getItemHeight()));

                    r.removeFromTop (seperatorHeight);
                }

                deviceSelector.setBounds (r);
        */
    }
    
    pub fn child_bounds_changed(&mut self, child_comp: *mut Component)  {
        
        todo!();
        /*
            if (! isResizing && childComp == &deviceSelector)
                    setToRecommendedSize();
        */
    }
    
    pub fn set_to_recommended_size(&mut self)  {
        
        todo!();
        /*
            const auto extraHeight = [&]
                {
                    if (! owner.getProcessorHasPotentialFeedbackLoop())
                        return 0;

                    const auto itemHeight = deviceSelector.getItemHeight();
                    const auto separatorHeight = (itemHeight >> 1);
                    return itemHeight + separatorHeight;
                }();

                setSize (getWidth(), deviceSelector.getHeight() + extraHeight);
        */
    }
}
