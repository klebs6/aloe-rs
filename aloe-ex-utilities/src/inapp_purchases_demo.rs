crate::ix!();

#[no_copy]
#[leak_detector]
pub struct InAppPurchasesDemo<'a> {
    base:                Component<'a>,
    base2:               AsyncUpdater<'a>,
    sound_names:         StringArray,
    phrase_label:        Label<'a>,              //{ "phraseLabel", NEEDS_TRANS ("Phrases:") };
    phrase_list_box:     ListBox<'a>,            // default = { "phraseListBox"  }
    phrase_model:        Box<ListBoxModel>,  //{ new PhraseModel() };
    play_stop_button:    TextButton<'a>,         // default = { "Play"  }
    player:              SoundPlayer,
    purchases:           VoicePurchases<'a>,     //{ *this };
    dm:                  AudioDeviceManager<'a>,
    voice_label:         Label<'a>,              //{ "voiceLabel", NEEDS_TRANS ("Voices:") };
    voice_list_box:      ListBox<'a>,            // default = { "voiceListBox"  }
    voice_model:         Box<VoiceModel<'a>>,    //{ new VoiceModel (purchases) };
    current_phrase_data: MemoryBlock,
}

impl<'a> Default for InAppPurchasesDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            Desktop::getInstance().getDefaultLookAndFeel().setUsingNativeAlertWindows (true);

            dm.addAudioCallback (&player);
            dm.initialiseWithDefaultDevices (0, 2);

            setOpaque (true);

            phraseListBox.setModel (phraseModel.get());
            voiceListBox .setModel (voiceModel.get());

            phraseListBox.setRowHeight (33);
            phraseListBox.selectRow (0);
            phraseListBox.updateContent();

            voiceListBox.setRowHeight (66);
            voiceListBox.selectRow (0);
            voiceListBox.updateContent();
            voiceListBox.getViewport()->setScrollOnDragEnabled (true);

            addAndMakeVisible (phraseLabel);
            addAndMakeVisible (phraseListBox);
            addAndMakeVisible (playStopButton);
            addAndMakeVisible (voiceLabel);
            addAndMakeVisible (voiceListBox);

            playStopButton.onClick = [this] { playStopPhrase(); };

            soundNames = purchases.getVoiceNames();

           #if ALOE_ANDROID || ALOE_IOS
            auto screenBounds = Desktop::getInstance().getDisplays().getPrimaryDisplay()->userArea;
            setSize (screenBounds.getWidth(), screenBounds.getHeight());
           #else
            setSize (800, 600);
           #endi
        */
    }
}

impl<'a> Drop for InAppPurchasesDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            dm.closeAudioDevice();
            dm.removeAudioCallback (&player);
         */
    }
}

impl<'a> InAppPurchasesDemo<'a> {

    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            voiceListBox.updateContent();
            voiceListBox.setEnabled (! purchases.isPurchaseInProgress());
            voiceListBox.repaint();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (20);

            {
                auto phraseArea = r.removeFromTop (r.getHeight() / 2);

                phraseLabel   .setBounds (phraseArea.removeFromTop (36).reduced (0, 10));
                playStopButton.setBounds (phraseArea.removeFromBottom (50).reduced (0, 10));
                phraseListBox .setBounds (phraseArea);
            }

            {
                auto voiceArea = r;

                voiceLabel  .setBounds (voiceArea.removeFromTop (36).reduced (0, 10));
                voiceListBox.setBounds (voiceArea);
            }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Desktop::getInstance().getDefaultLookAndFeel()
                          .findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn play_stop_phrase(&mut self)  {
        
        todo!();
        /*
            auto idx = voiceListBox.getSelectedRow();
            if (isPositiveAndBelow (idx, soundNames.size()))
            {
                auto assetName = "Purchases/" + soundNames[idx] + String (phraseListBox.getSelectedRow()) + ".ogg";

                if (auto fileStream = createAssetInputStream (assetName.toRawUTF8()))
                {
                    currentPhraseData.reset();
                    fileStream->readIntoMemoryBlock (currentPhraseData);

                    player.play (currentPhraseData.getData(), currentPhraseData.getSize());
                }
            }
        */
    }
}
