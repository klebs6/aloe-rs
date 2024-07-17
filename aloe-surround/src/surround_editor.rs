crate::ix!();

pub struct SurroundEditor<'a> {
    base:                   AudioProcessorEditor<'a>,
    base2:                  Timer,
    current_channel_layout: AudioChannelSet,
    no_channels_label:      Label<'a>, // default = { "noChannelsLabel", "Input disabled"  }
    layout_title:           Label<'a>,
    channel_buttons:        Vec<Box<TextButton<'a>>>,
    active_channels:        Vec<bool>,
    last_suspended:         bool,
}

impl<'a> SurroundEditor<'a> {

    pub fn new(parent: &mut AudioProcessor) -> Self {
    
        todo!();
        /*


            : AudioProcessorEditor (parent),
              currentChannelLayout (AudioChannelSet::disabled()),
              layoutTitle          ("LayoutTitleLabel", getLayoutName())

            layoutTitle.setJustificationType (Justification::centred);
            addAndMakeVisible (layoutTitle);
            addAndMakeVisible (noChannelsLabel);

            setSize (600, 100);

            lastSuspended = ! getAudioProcessor()->isSuspended();
            timerCallback();
            startTimer (500);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds();

            layoutTitle.setBounds (r.removeFromBottom (16));

            noChannelsLabel.setBounds (r);

            if (channelButtons.size() > 0)
            {
                auto buttonWidth = r.getWidth() / channelButtons.size();
                for (auto channelButton : channelButtons)
                    channelButton->setBounds (r.removeFromLeft (buttonWidth));
            }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn update_button(&mut self, btn: *mut Button)  {
        
        todo!();
        /*
            if (auto* textButton = dynamic_cast<TextButton*> (btn))
            {
                auto channelIndex = channelButtons.indexOf (textButton);

                if (auto* listener = dynamic_cast<ChannelClickListener*> (getAudioProcessor()))
                    listener->channelButtonClicked (channelIndex);
            }
        */
    }
    
    pub fn updategui(&mut self)  {
        
        todo!();
        /*
            const auto& channelSet = getAudioProcessor()->getChannelLayoutOfBus (false, 0);

            if (channelSet != currentChannelLayout)
            {
                currentChannelLayout = channelSet;

                layoutTitle.setText (currentChannelLayout.getDescription(), NotificationType::dontSendNotification);
                channelButtons.clear();
                activeChannels.resize (currentChannelLayout.size());

                if (currentChannelLayout == AudioChannelSet::disabled())
                {
                    noChannelsLabel.setVisible (true);
                }
                else
                {
                    auto numChannels = currentChannelLayout.size();

                    for (auto i = 0; i < numChannels; ++i)
                    {
                        auto channelName =
                        AudioChannelSet::getAbbreviatedChannelTypeName (currentChannelLayout.getTypeOfChannel (i));

                        TextButton* newButton;
                        channelButtons.add (newButton = new TextButton (channelName, channelName));

                        newButton->onClick = [this, newButton] { updateButton (newButton); };
                        addAndMakeVisible (newButton);
                    }

                    noChannelsLabel.setVisible (false);
                    resized();
                }

                if (auto* listener = dynamic_cast<ChannelClickListener*> (getAudioProcessor()))
                {
                    auto   activeColour = getLookAndFeel().findColour (Slider::thumbColourId);
                    auto inactiveColour = getLookAndFeel().findColour (Slider::trackColourId);

                    for (auto i = 0; i < activeChannels.size(); ++i)
                    {
                        auto isActive = listener->isChannelActive (i);
                        activeChannels.getReference (i) = isActive;
                        channelButtons[i]->setColour (TextButton::buttonColourId, isActive ? activeColour : inactiveColour);
                        channelButtons[i]->repaint();
                    }
                }
            }
        */
    }
    
    pub fn get_layout_name(&self) -> String {
        
        todo!();
        /*
            if (auto* p = getAudioProcessor())
                return p->getChannelLayoutOfBus (false, 0).getDescription();

            return "Unknown";
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (getAudioProcessor()->isSuspended() != lastSuspended)
            {
                lastSuspended = getAudioProcessor()->isSuspended();
                updateGUI();
            }

            if (! lastSuspended)
            {
                if (auto* listener = dynamic_cast<ChannelClickListener*> (getAudioProcessor()))
                {
                    auto   activeColour = getLookAndFeel().findColour (Slider::thumbColourId);
                    auto inactiveColour = getLookAndFeel().findColour (Slider::trackColourId);

                    for (auto i = 0; i < activeChannels.size(); ++i)
                    {
                        auto isActive = listener->isChannelActive (i);
                        if (activeChannels.getReference (i) != isActive)
                        {
                            activeChannels.getReference (i) = isActive;
                            channelButtons[i]->setColour (TextButton::buttonColourId, isActive ? activeColour : inactiveColour);
                            channelButtons[i]->repaint();
                        }
                    }
                }
            }
        */
    }
}
