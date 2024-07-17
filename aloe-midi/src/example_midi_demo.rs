crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/MidiDemo.h]

#[no_copy]
#[leak_detector]
pub struct MidiDemo<'a> {
    base:                 Component<'a>,
    base2:                Timer,
    base5:                AsyncUpdater<'a>,
    midi_input_label:     Label<'a>, // { "Midi Input Label",  "MIDI Input:" };
    midi_output_label:    Label<'a>, // { "Midi Output Label", "MIDI Output:" };
    incoming_midi_label:  Label<'a>, // { "Incoming Midi Label", "Received MIDI messages:" };
    outgoing_midi_label:  Label<'a>, // { "Outgoing Midi Label", "Play the keyboard to send MIDI messages..." };
    keyboard_state:       MidiKeyboardState,
    midi_keyboard:        MidiKeyboardComponent<'a>,
    midi_monitor:         TextEditor<'a>, // { "MIDI Monitor" };
    pair_button:          TextButton<'a>, // { "MIDI Bluetooth devices..." };
    midi_input_selector:  Box<MidiDeviceListBox<'a>>,
    midi_output_selector: Box<MidiDeviceListBox<'a>>,
    midi_inputs:          ReferenceCountedArray<MidiDeviceListEntry>,
    midi_outputs:         ReferenceCountedArray<MidiDeviceListEntry>,
    midi_monitor_lock:    CriticalSection,
    incoming_messages:    Vec<MidiMessage>,
}

impl<'a> MidiKeyboardStateListener for MidiDemo<'a> {

    fn handle_note_on(&mut self, 
        _0:               *mut MidiKeyboardState,
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            MidiMessage m (MidiMessage::noteOn (midiChannel, midiNoteNumber, velocity));
            m.setTimeStamp (Time::getMillisecondCounterHiRes() * 0.001);
            sendToOutputs (m);
        */
    }
    
    fn handle_note_off(&mut self, 
        _0:               *mut MidiKeyboardState,
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            MidiMessage m (MidiMessage::noteOff (midiChannel, midiNoteNumber, velocity));
            m.setTimeStamp (Time::getMillisecondCounterHiRes() * 0.001);
            sendToOutputs (m);
        */
    }
}

impl<'a> MidiInputCallback for MidiDemo<'a> {

    fn handle_incoming_midi_message(&mut self, 
        source:  *mut MidiInput,
        message: &MidiMessage)  {
        
        todo!();
        /*
            // This is called on the MIDI thread
            const ScopedLock sl (midiMonitorLock);
            incomingMessages.add (message);
            triggerAsyncUpdate();
        */
    }
}

impl<'a> Default for MidiDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*
            : midiKeyboard       (keyboardState, MidiKeyboardComponent::horizontalKeyboard),
              midiInputSelector  (new MidiDeviceListBox ("Midi Input Selector",  *this, true)),
              midiOutputSelector (new MidiDeviceListBox ("Midi Output Selector", *this, false))

            addLabelAndSetStyle (midiInputLabel);
            addLabelAndSetStyle (midiOutputLabel);
            addLabelAndSetStyle (incomingMidiLabel);
            addLabelAndSetStyle (outgoingMidiLabel);

            midiKeyboard.setName ("MIDI Keyboard");
            addAndMakeVisible (midiKeyboard);

            midiMonitor.setMultiLine (true);
            midiMonitor.setReturnKeyStartsNewLine (false);
            midiMonitor.setReadOnly (true);
            midiMonitor.setScrollbarsShown (true);
            midiMonitor.setCaretVisible (false);
            midiMonitor.setPopupMenuEnabled (false);
            midiMonitor.setText ({});
            addAndMakeVisible (midiMonitor);

            if (! BluetoothMidiDevicePairingDialogue::isAvailable())
                pairButton.setEnabled (false);

            addAndMakeVisible (pairButton);
            pairButton.onClick = []
            {
                RuntimePermissions::request (RuntimePermissions::bluetoothMidi,
                                             [] (bool wasGranted)
                                             {
                                                 if (wasGranted)
                                                     BluetoothMidiDevicePairingDialogue::open();
                                             });
            };
            keyboardState.addListener (this);

            addAndMakeVisible (midiInputSelector .get());
            addAndMakeVisible (midiOutputSelector.get());

            setSize (732, 520);

            startTimer (500)
        */
    }
}

impl<'a> Drop for MidiDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            stopTimer();
            midiInputs .clear();
            midiOutputs.clear();
            keyboardState.removeListener (this);

            midiInputSelector .reset();
            midiOutputSelector.reset();
         */
    }
}

impl<'a> MidiDemo<'a> {

    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            updateDeviceList (true);
            updateDeviceList (false);
        */
    }

    pub fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto margin = 10;

            midiInputLabel.setBounds (margin, margin,
                                      (getWidth() / 2) - (2 * margin), 24);

            midiOutputLabel.setBounds ((getWidth() / 2) + margin, margin,
                                       (getWidth() / 2) - (2 * margin), 24);

            midiInputSelector->setBounds (margin, (2 * margin) + 24,
                                          (getWidth() / 2) - (2 * margin),
                                          (getHeight() / 2) - ((4 * margin) + 24 + 24));

            midiOutputSelector->setBounds ((getWidth() / 2) + margin, (2 * margin) + 24,
                                           (getWidth() / 2) - (2 * margin),
                                           (getHeight() / 2) - ((4 * margin) + 24 + 24));

            pairButton.setBounds (margin, (getHeight() / 2) - (margin + 24),
                                  getWidth() - (2 * margin), 24);

            outgoingMidiLabel.setBounds (margin, getHeight() / 2, getWidth() - (2 * margin), 24);
            midiKeyboard.setBounds (margin, (getHeight() / 2) + (24 + margin), getWidth() - (2 * margin), 64);

            incomingMidiLabel.setBounds (margin, (getHeight() / 2) + (24 + (2 * margin) + 64),
                                         getWidth() - (2 * margin), 24);

            auto y = (getHeight() / 2) + ((2 * 24) + (3 * margin) + 64);
            midiMonitor.setBounds (margin, y,
                                   getWidth() - (2 * margin), getHeight() - y - margin);
        */
    }
    
    pub fn open_device(&mut self, 
        is_input: bool,
        index:    i32)  {
        
        todo!();
        /*
            if (isInput)
            {
                jassert (midiInputs[index]->inDevice.get() == nullptr);
                midiInputs[index]->inDevice = MidiInput::openDevice (midiInputs[index]->deviceInfo.identifier, this);

                if (midiInputs[index]->inDevice.get() == nullptr)
                {
                    DBG ("MidiDemo::openDevice: open input device for index = " << index << " failed!");
                    return;
                }

                midiInputs[index]->inDevice->start();
            }
            else
            {
                jassert (midiOutputs[index]->outDevice.get() == nullptr);
                midiOutputs[index]->outDevice = MidiOutput::openDevice (midiOutputs[index]->deviceInfo.identifier);

                if (midiOutputs[index]->outDevice.get() == nullptr)
                {
                    DBG ("MidiDemo::openDevice: open output device for index = " << index << " failed!");
                }
            }
        */
    }
    
    pub fn close_device(&mut self, 
        is_input: bool,
        index:    i32)  {
        
        todo!();
        /*
            if (isInput)
            {
                jassert (midiInputs[index]->inDevice.get() != nullptr);
                midiInputs[index]->inDevice->stop();
                midiInputs[index]->inDevice.reset();
            }
            else
            {
                jassert (midiOutputs[index]->outDevice.get() != nullptr);
                midiOutputs[index]->outDevice.reset();
            }
        */
    }
    
    pub fn get_num_midi_inputs(&self) -> i32 {
        
        todo!();
        /*
            return midiInputs.size();
        */
    }
    
    pub fn get_num_midi_outputs(&self) -> i32 {
        
        todo!();
        /*
            return midiOutputs.size();
        */
    }
    
    pub fn get_midi_device(&self, 
        index:    i32,
        is_input: bool) -> ReferenceCountedObjectPtr<MidiDeviceListEntry> {
        
        todo!();
        /*
            return isInput ? midiInputs[index] : midiOutputs[index];
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            // This is called on the message loop
            Vec<MidiMessage> messages;

            {
                const ScopedLock sl (midiMonitorLock);
                messages.swapWith (incomingMessages);
            }

            String messageText;

            for (auto& m : messages)
                messageText << m.getDescription() << "\n";

            midiMonitor.insertTextAtCaret (messageText);
        */
    }
    
    pub fn send_to_outputs(&mut self, msg: &MidiMessage)  {
        
        todo!();
        /*
            for (auto midiOutput : midiOutputs)
                if (midiOutput->outDevice.get() != nullptr)
                    midiOutput->outDevice->sendMessageNow (msg);
        */
    }
    
    pub fn has_device_list_changed(&mut self, 
        available_devices: &[MidiDeviceInfo],
        is_input_device:   bool) -> bool {
        
        todo!();
        /*
            ReferenceCountedArray<MidiDeviceListEntry>& midiDevices = isInputDevice ? midiInputs
                                                                                    : midiOutputs;

            if (availableDevices.size() != midiDevices.size())
                return true;

            for (auto i = 0; i < availableDevices.size(); ++i)
                if (availableDevices[i] != midiDevices[i]->deviceInfo)
                    return true;

            return false;
        */
    }
    
    pub fn find_device(&self, 
        device:          MidiDeviceInfo,
        is_input_device: bool) -> ReferenceCountedObjectPtr<MidiDeviceListEntry> {
        
        todo!();
        /*
            const ReferenceCountedArray<MidiDeviceListEntry>& midiDevices = isInputDevice ? midiInputs
                                                                                          : midiOutputs;

            for (auto& d : midiDevices)
                if (d->deviceInfo == device)
                    return d;

            return nullptr;
        */
    }
    
    pub fn close_unplugged_devices(&mut self, 
        currently_plugged_in_devices: &[MidiDeviceInfo],
        is_input_device:              bool)  {
        
        todo!();
        /*
            ReferenceCountedArray<MidiDeviceListEntry>& midiDevices = isInputDevice ? midiInputs
                                                                                    : midiOutputs;

            for (auto i = midiDevices.size(); --i >= 0;)
            {
                auto& d = *midiDevices[i];

                if (! currentlyPluggedInDevices.contains (d.deviceInfo))
                {
                    if (isInputDevice ? d.inDevice .get() != nullptr
                                      : d.outDevice.get() != nullptr)
                        closeDevice (isInputDevice, i);

                    midiDevices.remove (i);
                }
            }
        */
    }
    
    pub fn update_device_list(&mut self, is_input_device_list: bool)  {
        
        todo!();
        /*
            auto availableDevices = isInputDeviceList ? MidiInput::getAvailableDevices()
                                                      : MidiOutput::getAvailableDevices();

            if (hasDeviceListChanged (availableDevices, isInputDeviceList))
            {

                ReferenceCountedArray<MidiDeviceListEntry>& midiDevices
                    = isInputDeviceList ? midiInputs : midiOutputs;

                closeUnpluggedDevices (availableDevices, isInputDeviceList);

                ReferenceCountedArray<MidiDeviceListEntry> newDeviceList;

                // add all currently plugged-in devices to the device list
                for (auto& newDevice : availableDevices)
                {
                    MidiDeviceListEntry::Ptr entry = findDevice (newDevice, isInputDeviceList);

                    if (entry == nullptr)
                        entry = new MidiDeviceListEntry (newDevice);

                    newDeviceList.add (entry);
                }

                // actually update the device list
                midiDevices = newDeviceList;

                // update the selection status of the combo-box
                if (auto* midiSelector = isInputDeviceList ? midiInputSelector.get() : midiOutputSelector.get())
                    midiSelector->syncSelectedItemsWithDeviceList (midiDevices);
            }
        */
    }
    
    pub fn add_label_and_set_style(&mut self, label: &mut Label)  {
        
        todo!();
        /*
            label.setFont (Font (15.00f, Font::plain));
            label.setJustificationType (Justification::centredLeft);
            label.setEditable (false, false, false);
            label.setColour (TextEditor::textColourId, Colours::black);
            label.setColour (TextEditor::backgroundColourId, Colour (0x00000000));

            addAndMakeVisible (label);
        */
    }
}
