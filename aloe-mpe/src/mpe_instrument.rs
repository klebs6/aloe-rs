crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEInstrument.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEInstrument.cpp]

lazy_static!{
    /*
    const uint8 noLSBValueReceived = 0xff;
    const Range<int> allChannels { 1, 17 };
    */
}

pub fn mpe_instrument_fill<Range, Value>(
        range: &mut Range,
        value: &Value)  {

    todo!();
    /*
        std::fill (std::begin (range), std::end (range), value);
    */
}

/**
  | This class represents an instrument
  | handling MPE.
  | 
  | It has an MPE zone layout and maintains
  | a state of currently active (playing)
  | notes and the values of their dimensions
  | of expression.
  | 
  | You can trigger and modulate notes:
  | 
  | - by passing MIDI messages with the method
  | processNextMidiEvent;
  | 
  | - by directly calling the methods noteOn,
  | noteOff etc.
  | 
  | The class implements the channel and
  | note management logic specified in
  | MPE. If you pass it a message, it will
  | know what notes on what channels (if
  | any) should be affected by that message.
  | 
  | The class has a MpeInstrumentListener class with the
  | three callbacks MPENoteAdded, MPENoteChanged,
  | and MPENoteFinished. Implement such
  | a MpeInstrumentListener class to react to note changes
  | and trigger some functionality for
  | your application that depends on the
  | MPE note state. For example, you can
  | use this class to write an MPE visualiser.
  | 
  | If you want to write a real-time audio
  | synth with MPE functionality, you should
  | instead use the classes MPESynthesizerBase,
  | which adds the ability to render audio
  | and to manage voices.
  | 
  | @see MPENote, MPEZoneLayout, MPESynthesizer
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MPEInstrument {
    lock:                                        CriticalSection,
    notes:                                       Vec<MPENote>,
    zone_layout:                                 MPEZoneLayout,
    listeners:                                   ListenerList<Box<dyn MpeInstrumentListener>>,
    last_pressure_lower_bit_received_on_channel: [u8; 16],
    last_timbre_lower_bit_received_on_channel:   [u8; 16],
    is_member_channel_sustained:                 [bool; 16],
    legacy_mode:                                 MpeInstrumentLegacyMode,
    pitchbend_dimension:                         MpeInstrumentMPEDimension,
    pressure_dimension:                          MpeInstrumentMPEDimension,
    timbre_dimension:                            MpeInstrumentMPEDimension,
}

impl Default for MPEInstrument {
    
    /**
      | Constructor.
      | 
      | This will construct an MPE instrument
      | with inactive lower and upper zones.
      | 
      | In order to process incoming MIDI, call
      | setZoneLayout, define the layout via
      | MIDI RPN messages, or set the instrument
      | to legacy mode.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*


            mpeInstrumentFill (lastPressureLowerBitReceivedOnChannel, noLSBValueReceived);
        mpeInstrumentFill (lastTimbreLowerBitReceivedOnChannel, noLSBValueReceived);
        mpeInstrumentFill (isMemberChannelSustained, false);

        pitchbendDimension.value = &MPENote::pitchbend;
        pressureDimension.value = &MPENote::pressure;
        timbreDimension.value = &MPENote::timbre;

        resetLastReceivedValues();

        legacyMode.isEnabled = false;
        legacyMode.pitchbendRange = 2;
        legacyMode.channelRange = allChannels;
        */
    }
}

impl MPEInstrument {
    
    /**
      | Returns the current zone layout of the
      | instrument. This happens by value,
      | to enforce thread-safety and class
      | invariants.
      | 
      | Note: If the instrument is in legacy
      | mode, the return value of this method
      | is unspecified.
      |
      */
    pub fn get_zone_layout(&self) -> MPEZoneLayout {
        
        todo!();
        /*
            return zoneLayout;
        */
    }
    
    pub fn reset_last_received_values(&mut self)  {
        
        todo!();
        /*
            struct Defaults
        {
            MpeInstrumentMPEDimension& dimension;
            MPEValue defaultValue;
        };

        // The default value for pressure is 0, for all other dimensions it is centre
        for (const auto& pair : { Defaults { pressureDimension,  MPEValue::minValue() },
                                  Defaults { pitchbendDimension, MPEValue::centreValue() },
                                  Defaults { timbreDimension,    MPEValue::centreValue() } })
        {
            mpeInstrumentFill (pair.dimension.lastValueReceivedOnChannel, pair.defaultValue);
        }
        */
    }
    
    /**
      | Re-sets the zone layout of the instrument
      | to the one passed in. As a side effect,
      | this will discard all currently playing
      | notes, and call noteReleased for all
      | of them.
      | 
      | This will also disable legacy mode in
      | case it was enabled previously.
      |
      */
    pub fn set_zone_layout(&mut self, new_layout: MPEZoneLayout)  {
        
        todo!();
        /*
            releaseAllNotes();

        const ScopedLock sl (lock);
        legacyMode.isEnabled = false;
        zoneLayout = newLayout;

        resetLastReceivedValues();
        */
    }
    
    /**
      | Puts the instrument into legacy mode.
      | As a side effect, this will discard all
      | currently playing notes, and call noteReleased
      | for all of them.
      | 
      | This special zone layout mode is for
      | backwards compatibility with non-MPE
      | MIDI devices. In this mode, the instrument
      | will ignore the current MPE zone layout.
      | It will instead take a range of MIDI channels
      | (default: all channels 1-16) and treat
      | them as note channels, with no master
      | channel. MIDI channels outside of this
      | range will be ignored.
      | 
      | -----------
      | @param pitchbendRange
      | 
      | The note pitchbend range in semitones
      | to use when in legacy mode. Must be between
      | 0 and 96, otherwise behaviour is undefined.
      | The default pitchbend range in legacy
      | mode is +/- 2 semitones.
      | ----------
      | @param channelRange
      | 
      | The range of MIDI channels to use for
      | notes when in legacy mode. The default
      | is to use all MIDI channels (1-16).
      | 
      | To get out of legacy mode, set a new MPE
      | zone layout using setZoneLayout.
      |
      */
    pub fn enable_legacy_mode(
        &mut self, 
        pitchbend_range: Option<i32>,
        channel_range:   Option<Range<i32>>
    ) {

        let pitchbend_range = pitchbend_range.unwrap_or(2);

        let channel_range = channel_range.unwrap_or(1..17);
        
        todo!();
        /*
            releaseAllNotes();

        const ScopedLock sl (lock);
        legacyMode.isEnabled = true;
        legacyMode.pitchbendRange = pitchbendRange;
        legacyMode.channelRange = channelRange;
        zoneLayout.clearAllZones();
        */
    }
    
    /**
      | Returns true if the instrument is in
      | legacy mode, false otherwise.
      |
      */
    pub fn is_legacy_mode_enabled(&self) -> bool {
        
        todo!();
        /*
            return legacyMode.isEnabled;
        */
    }
    
    /**
      | Returns the range of MIDI channels (1-16)
      | to be used for notes when in legacy mode.
      |
      */
    pub fn get_legacy_mode_channel_range(&self) -> Range<i32> {
        
        todo!();
        /*
            return legacyMode.channelRange;
        */
    }
    
    /**
      | Re-sets the range of MIDI channels (1-16)
      | to be used for notes when in legacy mode.
      |
      */
    pub fn set_legacy_mode_channel_range(&mut self, channel_range: Range<i32>)  {
        
        todo!();
        /*
            jassert (allChannels.contains (channelRange));

        releaseAllNotes();
        const ScopedLock sl (lock);
        legacyMode.channelRange = channelRange;
        */
    }
    
    /**
      | Returns the pitchbend range in semitones
      | (0-96) to be used for notes when in legacy
      | mode.
      |
      */
    pub fn get_legacy_mode_pitchbend_range(&self) -> i32 {
        
        todo!();
        /*
            return legacyMode.pitchbendRange;
        */
    }
    
    /**
      | Re-sets the pitchbend range in semitones
      | (0-96) to be used for notes when in legacy
      | mode.
      |
      */
    pub fn set_legacy_mode_pitchbend_range(&mut self, pitchbend_range: i32)  {
        
        todo!();
        /*
            jassert (pitchbendRange >= 0 && pitchbendRange <= 96);

        releaseAllNotes();
        const ScopedLock sl (lock);
        legacyMode.pitchbendRange = pitchbendRange;
        */
    }
    
    /**
      | Set the MPE tracking mode for the pressure
      | dimension.
      |
      */
    pub fn set_pressure_tracking_mode(&mut self, mode_to_use: MpeInstrumentTrackingMode)  {
        
        todo!();
        /*
            pressureDimension.trackingMode = modeToUse;
        */
    }
    
    /**
      | Set the MPE tracking mode for the pitchbend
      | dimension.
      |
      */
    pub fn set_pitchbend_tracking_mode(&mut self, mode_to_use: MpeInstrumentTrackingMode)  {
        
        todo!();
        /*
            pitchbendDimension.trackingMode = modeToUse;
        */
    }
    
    /**
      | Set the MPE tracking mode for the timbre
      | dimension.
      |
      */
    pub fn set_timbre_tracking_mode(&mut self, mode_to_use: MpeInstrumentTrackingMode)  {
        
        todo!();
        /*
            timbreDimension.trackingMode = modeToUse;
        */
    }
    
    /**
      | Adds a listener.
      |
      */
    pub fn add_listener(&mut self, listener_to_add: *mut dyn MpeInstrumentListener)  {
        
        todo!();
        /*
            listeners.add (listenerToAdd);
        */
    }
    
    /**
      | Removes a listener.
      |
      */
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn MpeInstrumentListener)  {
        
        todo!();
        /*
            listeners.remove (listenerToRemove);
        */
    }
    
    /**
      | Process a MIDI message and trigger the
      | appropriate method calls (noteOn,
      | noteOff etc.)
      | 
      | You can override this method if you need
      | some special MIDI message treatment
      | on top of the standard MPE logic implemented
      | here.
      |
      */
    pub fn process_next_midi_event(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            zoneLayout.processNextMidiEvent (message);

        if (message.isNoteOn (true))              processMidiNoteOnMessage (message);
        else if (message.isNoteOff (false))       processMidiNoteOffMessage (message);
        else if (message.isResetAllControllers()
                 || message.isAllNotesOff())      processMidiResetAllControllersMessage (message);
        else if (message.isPitchWheel())          processMidiPitchWheelMessage (message);
        else if (message.isChannelPressure())     processMidiChannelPressureMessage (message);
        else if (message.isController())          processMidiControllerMessage (message);
        else if (message.isAftertouch())          processMidiAfterTouchMessage (message);
        */
    }
    
    pub fn process_midi_note_on_message(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            // Note: If a note-on with velocity = 0 is used to convey a note-off,
        // then the actual note-off velocity is not known. In this case,
        // the MPE convention is to use note-off velocity = 64.

        if (message.getVelocity() == 0)
        {
            noteOff (message.getChannel(),
                     message.getNoteNumber(),
                     MPEValue::from7BitInt (64));
        }
        else
        {
            noteOn (message.getChannel(),
                    message.getNoteNumber(),
                    MPEValue::from7BitInt (message.getVelocity()));
        }
        */
    }
    
    pub fn process_midi_note_off_message(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            noteOff (message.getChannel(),
                 message.getNoteNumber(),
                 MPEValue::from7BitInt (message.getVelocity()));
        */
    }
    
    pub fn process_midi_pitch_wheel_message(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            pitchbend (message.getChannel(),
                   MPEValue::from14BitInt (message.getPitchWheelValue()));
        */
    }
    
    pub fn process_midi_channel_pressure_message(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            pressure (message.getChannel(),
                  MPEValue::from7BitInt (message.getChannelPressureValue()));
        */
    }
    
    pub fn process_midi_controller_message(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            switch (message.getControllerNumber())
        {
            case 64:    sustainPedal      (message.getChannel(), message.isSustainPedalOn());   break;
            case 66:    sostenutoPedal    (message.getChannel(), message.isSostenutoPedalOn()); break;
            case 70:    handlePressureMSB (message.getChannel(), message.getControllerValue()); break;
            case 74:    handleTimbreMSB   (message.getChannel(), message.getControllerValue()); break;
            case 102:   handlePressureLSB (message.getChannel(), message.getControllerValue()); break;
            case 106:   handleTimbreLSB   (message.getChannel(), message.getControllerValue()); break;
            default:    break;
        }
        */
    }
    
    pub fn process_midi_reset_all_controllers_message(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            // in MPE mode, "reset all controllers" is per-zone and expected on the master channel;
        // in legacy mode, it is per MIDI channel (within the channel range used).

        if (legacyMode.isEnabled && legacyMode.channelRange.contains (message.getChannel()))
        {
            for (auto i = notes.size(); --i >= 0;)
            {
                auto& note = notes.getReference (i);

                if (note.midiChannel == message.getChannel())
                {
                    note.keyState = MPENote::off;
                    note.noteOffVelocity = MPEValue::from7BitInt (64); // some reasonable number
                    listeners.call ([&] (MpeInstrumentListener& l) { l.noteReleased (note); });
                    notes.remove (i);
                }
            }
        }
        else if (isMasterChannel (message.getChannel()))
        {
            auto zone = (message.getChannel() == 1 ? zoneLayout.getLowerZone()
                                                   : zoneLayout.getUpperZone());

            for (auto i = notes.size(); --i >= 0;)
            {
                auto& note = notes.getReference (i);

                if (zone.isUsing (note.midiChannel))
                {
                    note.keyState = MPENote::off;
                    note.noteOffVelocity = MPEValue::from7BitInt (64); // some reasonable number
                    listeners.call ([&] (MpeInstrumentListener& l) { l.noteReleased (note); });
                    notes.remove (i);
                }
            }
        }
        */
    }
    
    pub fn process_midi_after_touch_message(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            if (! isMasterChannel (message.getChannel()))
            return;

        polyAftertouch (message.getChannel(), message.getNoteNumber(),
                        MPEValue::from7BitInt (message.getAfterTouchValue()));
        */
    }
    
    pub fn handle_pressuremsb(&mut self, 
        midi_channel: i32,
        value:        i32)  {
        
        todo!();
        /*
            auto lsb = lastPressureLowerBitReceivedOnChannel[midiChannel - 1];

        pressure (midiChannel, lsb == noLSBValueReceived ? MPEValue::from7BitInt (value)
                                                         : MPEValue::from14BitInt (lsb + (value << 7)));
        */
    }
    
    pub fn handle_pressurelsb(&mut self, 
        midi_channel: i32,
        value:        i32)  {
        
        todo!();
        /*
            lastPressureLowerBitReceivedOnChannel[midiChannel - 1] = uint8 (value);
        */
    }
    
    pub fn handle_timbremsb(&mut self, 
        midi_channel: i32,
        value:        i32)  {
        
        todo!();
        /*
            auto lsb = lastTimbreLowerBitReceivedOnChannel[midiChannel - 1];

        timbre (midiChannel, lsb == noLSBValueReceived ? MPEValue::from7BitInt (value)
                                                       : MPEValue::from14BitInt (lsb + (value << 7)));
        */
    }
    
    pub fn handle_timbrelsb(&mut self, 
        midi_channel: i32,
        value:        i32)  {
        
        todo!();
        /*
            lastTimbreLowerBitReceivedOnChannel[midiChannel - 1] = uint8 (value);
        */
    }
    
    /**
      | Request a note-on on the given channel,
      | with the given initial note number and
      | velocity.
      | 
      | If the message arrives on a valid note
      | channel, this will create a new MPENote
      | and call the noteAdded callback.
      |
      */
    pub fn note_on(&mut self, 
        midi_channel:          i32,
        midi_note_number:      i32,
        midi_note_on_velocity: MPEValue)  {
        
        todo!();
        /*
            if (! isUsingChannel (midiChannel))
            return;

        MPENote newNote (midiChannel,
                         midiNoteNumber,
                         midiNoteOnVelocity,
                         getInitialValueForNewNote (midiChannel, pitchbendDimension),
                         getInitialValueForNewNote (midiChannel, pressureDimension),
                         getInitialValueForNewNote (midiChannel, timbreDimension),
                         isMemberChannelSustained[midiChannel - 1] ? MPENote::keyDownAndSustained : MPENote::keyDown);

        const ScopedLock sl (lock);
        updateNoteTotalPitchbend (newNote);

        if (auto* alreadyPlayingNote = getNotePtr (midiChannel, midiNoteNumber))
        {
            // pathological case: second note-on received for same note -> retrigger it
            alreadyPlayingNote->keyState = MPENote::off;
            alreadyPlayingNote->noteOffVelocity = MPEValue::from7BitInt (64); // some reasonable number
            listeners.call ([=] (MpeInstrumentListener& l) { l.noteReleased (*alreadyPlayingNote); });
            notes.remove (alreadyPlayingNote);
        }

        notes.add (newNote);
        listeners.call ([&] (MpeInstrumentListener& l) { l.noteAdded (newNote); });
        */
    }
    
    /**
      | Request a note-off.
      | 
      | If there is a matching playing note,
      | this will release the note (except if
      | it is sustained by a sustain or sostenuto
      | pedal) and call the noteReleased callback.
      |
      */
    pub fn note_off(&mut self, 
        midi_channel:           i32,
        midi_note_number:       i32,
        midi_note_off_velocity: MPEValue)  {
        
        todo!();
        /*
            if (notes.isEmpty() || ! isUsingChannel (midiChannel))
            return;

        const ScopedLock sl (lock);

        if (auto* note = getNotePtr (midiChannel, midiNoteNumber))
        {
            note->keyState = (note->keyState == MPENote::keyDownAndSustained) ? MPENote::sustained : MPENote::off;
            note->noteOffVelocity = midiNoteOffVelocity;

            // If no more notes are playing on this channel in mpe mode, reset the dimension values
            if (! legacyMode.isEnabled && getLastNotePlayedPtr (midiChannel) == nullptr)
            {
                pressureDimension.lastValueReceivedOnChannel[midiChannel - 1] = MPEValue::minValue();
                pitchbendDimension.lastValueReceivedOnChannel[midiChannel - 1] = MPEValue::centreValue();
                timbreDimension.lastValueReceivedOnChannel[midiChannel - 1] = MPEValue::centreValue();
            }

            if (note->keyState == MPENote::off)
            {
                listeners.call ([=] (MpeInstrumentListener& l) { l.noteReleased (*note); });
                notes.remove (note);
            }
            else
            {
                listeners.call ([=] (MpeInstrumentListener& l) { l.noteKeyStateChanged (*note); });
            }
        }
        */
    }
    
    /**
      | Request a pitchbend on the given channel
      | with the given value (in units of MIDI
      | pitchwheel position).
      | 
      | Internally, this will determine whether
      | the pitchwheel move is a per-note pitchbend
      | or a master pitchbend (depending on
      | midiChannel), take the correct per-note
      | or master pitchbend range of the affected
      | MPE zone, and apply the resulting pitchbend
      | to the affected note(s) (if any).
      |
      */
    pub fn pitchbend(&mut self, 
        midi_channel: i32,
        value:        MPEValue)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        updateDimension (midiChannel, pitchbendDimension, value);
        */
    }
    
    /**
      | Request a pressure change on the given
      | channel with the given value.
      | 
      | This will modify the pressure dimension
      | of the note currently held down on this
      | channel (if any). If the channel is a
      | zone master channel, the pressure change
      | will be broadcast to all notes in this
      | zone.
      |
      */
    pub fn pressure(&mut self, 
        midi_channel: i32,
        value:        MPEValue)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        updateDimension (midiChannel, pressureDimension, value);
        */
    }
    
    /**
      | Request a third dimension (timbre)
      | change on the given channel with the
      | given value.
      | 
      | This will modify the timbre dimension
      | of the note currently held down on this
      | channel (if any). If the channel is a
      | zone master channel, the timbre change
      | will be broadcast to all notes in this
      | zone.
      |
      */
    pub fn timbre(&mut self, 
        midi_channel: i32,
        value:        MPEValue)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        updateDimension (midiChannel, timbreDimension, value);
        */
    }
    
    /**
      | Request a poly-aftertouch change for
      | a given note number.
      | 
      | The change will be broadcast to all notes
      | sharing the channel and note number
      | of the change message.
      |
      */
    pub fn poly_aftertouch(&mut self, 
        midi_channel:     i32,
        midi_note_number: i32,
        value:            MPEValue)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (auto i = notes.size(); --i >= 0;)
        {
            auto& note = notes.getReference (i);

            if (note.midiChannel == midiChannel
                && note.initialNote == midiNoteNumber
                && pressureDimension.getValue (note) != value)
            {
                pressureDimension.getValue (note) = value;
                callListenersDimensionChanged (note, pressureDimension);
            }
        }
        */
    }
    
    pub fn get_initial_value_for_new_note(&self, 
        midi_channel: i32,
        dimension:    &mut MpeInstrumentMPEDimension) -> MPEValue {
        
        todo!();
        /*
            if (! legacyMode.isEnabled && getLastNotePlayedPtr (midiChannel) != nullptr)
            return &dimension == &pressureDimension ? MPEValue::minValue() : MPEValue::centreValue();

        return dimension.lastValueReceivedOnChannel[midiChannel - 1];
        */
    }
    
    pub fn update_dimension(&mut self, 
        midi_channel: i32,
        dimension:    &mut MpeInstrumentMPEDimension,
        value:        MPEValue)  {
        
        todo!();
        /*
            dimension.lastValueReceivedOnChannel[midiChannel - 1] = value;

        if (notes.isEmpty())
            return;

        if (isMemberChannel (midiChannel))
        {
            if (dimension.trackingMode == allNotesOnChannel)
            {
                for (auto i = notes.size(); --i >= 0;)
                {
                    auto& note = notes.getReference (i);

                    if (note.midiChannel == midiChannel)
                        updateDimensionForNote (note, dimension, value);
                }
            }
            else
            {
                if (auto* note = getNotePtr (midiChannel, dimension.trackingMode))
                    updateDimensionForNote (*note, dimension, value);
            }
        }
        else if (isMasterChannel (midiChannel))
        {
            updateDimensionMaster (midiChannel == 1, dimension, value);
        }
        */
    }
    
    pub fn update_dimension_master(&mut self, 
        is_lower_zone: bool,
        dimension:     &mut MpeInstrumentMPEDimension,
        value:         MPEValue)  {
        
        todo!();
        /*
            auto zone = (isLowerZone ? zoneLayout.getLowerZone()
                                 : zoneLayout.getUpperZone());

        if (! zone.isActive())
            return;

        for (auto i = notes.size(); --i >= 0;)
        {
            auto& note = notes.getReference (i);

            if (! zone.isUsing (note.midiChannel))
                continue;

            if (&dimension == &pitchbendDimension)
            {
                // master pitchbend is a special case: we don't change the note's own pitchbend,
                // instead we have to update its total (master + note) pitchbend.
                updateNoteTotalPitchbend (note);
                listeners.call ([&] (MpeInstrumentListener& l) { l.notePitchbendChanged (note); });
            }
            else if (dimension.getValue (note) != value)
            {
                dimension.getValue (note) = value;
                callListenersDimensionChanged (note, dimension);
            }
        }
        */
    }
    
    pub fn update_dimension_for_note(&mut self, 
        note:      &mut MPENote,
        dimension: &mut MpeInstrumentMPEDimension,
        value:     MPEValue)  {
        
        todo!();
        /*
            if (dimension.getValue (note) != value)
        {
            dimension.getValue (note) = value;

            if (&dimension == &pitchbendDimension)
                updateNoteTotalPitchbend (note);

            callListenersDimensionChanged (note, dimension);
        }
        */
    }
    
    pub fn call_listeners_dimension_changed(&mut self, 
        note:      &MPENote,
        dimension: &MpeInstrumentMPEDimension)  {
        
        todo!();
        /*
            if (&dimension == &pressureDimension)  { listeners.call ([&] (MpeInstrumentListener& l) { l.notePressureChanged  (note); }); return; }
        if (&dimension == &timbreDimension)    { listeners.call ([&] (MpeInstrumentListener& l) { l.noteTimbreChanged    (note); }); return; }
        if (&dimension == &pitchbendDimension) { listeners.call ([&] (MpeInstrumentListener& l) { l.notePitchbendChanged (note); }); return; }
        */
    }
    
    pub fn update_note_total_pitchbend(&mut self, note: &mut MPENote)  {
        
        todo!();
        /*
            if (legacyMode.isEnabled)
        {
            note.totalPitchbendInSemitones = note.pitchbend.asSignedFloat() * (float) legacyMode.pitchbendRange;
        }
        else
        {
            auto zone = zoneLayout.getLowerZone();

            if (! zone.isActive() || ! zone.isUsing (note.midiChannel))
            {
                auto upperZone = zoneLayout.getUpperZone();

                if (upperZone.isActive() && upperZone.isUsing (note.midiChannel))
                {
                    zone = upperZone;
                }
                else
                {
                    // this note doesn't belong to any zone!
                    jassertfalse;
                    return;
                }
            }

            auto notePitchbendInSemitones = 0.0f;

            if (zone.isUsingChannelAsMemberChannel (note.midiChannel))
                notePitchbendInSemitones = note.pitchbend.asSignedFloat() * (float) zone.perNotePitchbendRange;

            auto masterPitchbendInSemitones = pitchbendDimension.lastValueReceivedOnChannel[zone.getMasterChannel() - 1]
                                                                .asSignedFloat()
                                              * (float) zone.masterPitchbendRange;

            note.totalPitchbendInSemitones = notePitchbendInSemitones + masterPitchbendInSemitones;
        }
        */
    }
    
    /**
      | Request a sustain pedal press or release.
      | 
      | If midiChannel is a zone's master channel,
      | this will act on all notes in that zone;
      | otherwise, nothing will happen.
      |
      */
    pub fn sustain_pedal(&mut self, 
        midi_channel: i32,
        is_down:      bool)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        handleSustainOrSostenuto (midiChannel, isDown, false);
        */
    }
    
    /**
      | Request a sostenuto pedal press or release.
      | 
      | If midiChannel is a zone's master channel,
      | this will act on all notes in that zone;
      | otherwise, nothing will happen.
      |
      */
    pub fn sostenuto_pedal(&mut self, 
        midi_channel: i32,
        is_down:      bool)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        handleSustainOrSostenuto (midiChannel, isDown, true);
        */
    }
    
    pub fn handle_sustain_or_sostenuto(&mut self, 
        midi_channel: i32,
        is_down:      bool,
        is_sostenuto: bool)  {
        
        todo!();
        /*
            // in MPE mode, sustain/sostenuto is per-zone and expected on the master channel;
        // in legacy mode, sustain/sostenuto is per MIDI channel (within the channel range used).

        if (legacyMode.isEnabled ? (! legacyMode.channelRange.contains (midiChannel)) : (! isMasterChannel (midiChannel)))
            return;

        auto zone = (midiChannel == 1 ? zoneLayout.getLowerZone()
                                      : zoneLayout.getUpperZone());

        for (auto i = notes.size(); --i >= 0;)
        {
            auto& note = notes.getReference (i);

            if (legacyMode.isEnabled ? (note.midiChannel == midiChannel) : zone.isUsing (note.midiChannel))
            {
                if (note.keyState == MPENote::keyDown && isDown)
                    note.keyState = MPENote::keyDownAndSustained;
                else if (note.keyState == MPENote::sustained && ! isDown)
                    note.keyState = MPENote::off;
                else if (note.keyState == MPENote::keyDownAndSustained && ! isDown)
                    note.keyState = MPENote::keyDown;

                if (note.keyState == MPENote::off)
                {
                    listeners.call ([&] (MpeInstrumentListener& l) { l.noteReleased (note); });
                    notes.remove (i);
                }
                else
                {
                    listeners.call ([&] (MpeInstrumentListener& l) { l.noteKeyStateChanged (note); });
                }
            }
        }

        if (! isSostenuto)
        {
            isMemberChannelSustained[midiChannel - 1] = isDown;

            if (! legacyMode.isEnabled)
            {
                if (zone.isLowerZone())
                    for (auto i = zone.getFirstMemberChannel(); i <= zone.getLastMemberChannel(); ++i)
                        isMemberChannelSustained[i - 1] = isDown;
                else
                    for (auto i = zone.getFirstMemberChannel(); i >= zone.getLastMemberChannel(); --i)
                        isMemberChannelSustained[i - 1] = isDown;
            }
        }
        */
    }
    
    /**
      | Returns true if the given MIDI channel
      | (1-16) is a note channel in any of the
      | MPEInstrument's MPE zones; false otherwise.
      | 
      | When in legacy mode, this will return
      | true if the given channel is contained
      | in the current legacy mode channel range;
      | false otherwise.
      |
      */
    pub fn is_member_channel(&self, midi_channel: i32) -> bool {
        
        todo!();
        /*
            if (legacyMode.isEnabled)
            return legacyMode.channelRange.contains (midiChannel);

        return zoneLayout.getLowerZone().isUsingChannelAsMemberChannel (midiChannel)
                || zoneLayout.getUpperZone().isUsingChannelAsMemberChannel (midiChannel);
        */
    }
    
    /**
      | Returns true if the given MIDI channel
      | (1-16) is a master channel (channel
      | 1 or 16).
      | 
      | In legacy mode, this will always return
      | false.
      |
      */
    pub fn is_master_channel(&self, midi_channel: i32) -> bool {
        
        todo!();
        /*
            if (legacyMode.isEnabled)
            return false;

        const auto lowerZone = zoneLayout.getLowerZone();
        const auto upperZone = zoneLayout.getUpperZone();

        return (lowerZone.isActive() && midiChannel == lowerZone.getMasterChannel())
                || (upperZone.isActive() && midiChannel == upperZone.getMasterChannel());
        */
    }
    
    /**
      | Returns true if the given MIDI channel
      | (1-16) is used by any of the MPEInstrument's
      | MPE zones; false otherwise.
      | 
      | When in legacy mode, this will return
      | true if the given channel is contained
      | in the current legacy mode channel range;
      | false otherwise.
      |
      */
    pub fn is_using_channel(&self, midi_channel: i32) -> bool {
        
        todo!();
        /*
            if (legacyMode.isEnabled)
            return legacyMode.channelRange.contains (midiChannel);

        return zoneLayout.getLowerZone().isUsing (midiChannel)
                || zoneLayout.getUpperZone().isUsing (midiChannel);
        */
    }
    
    /**
      | Returns the number of MPE notes currently
      | played by the instrument.
      |
      */
    pub fn get_num_playing_notes(&self) -> i32 {
        
        todo!();
        /*
            return notes.size();
        */
    }
    
    /**
      | Returns the note currently playing
      | on the given midiChannel with the specified
      | initial MIDI note number, if there is
      | such a note. Otherwise, this returns
      | an invalid MPENote (check with note.isValid()
      | before use!)
      |
      */
    pub fn get_note(&self, 
        midi_channel:     i32,
        midi_note_number: i32) -> MPENote {
        
        todo!();
        /*
            if (auto* note = getNotePtr (midiChannel, midiNoteNumber))
            return *note;

        return {};
        */
    }
    
    /**
      | Returns the note at the given index.
      | 
      | If there is no such note, returns an invalid
      | MPENote. The notes are sorted such that
      | the most recently added note is the last
      | element.
      |
      */
    pub fn get_note_by_index(&self, index: i32) -> MPENote {
        
        todo!();
        /*
            return notes[index];
        */
    }
    
    /**
      | Returns the most recent note that is
      | playing on the given midiChannel (this
      | will be the note which has received the
      | most recent note-on without a corresponding
      | note-off), if there is such a note. Otherwise,
      | this returns an invalid MPENote (check
      | with note.isValid() before use!)
      |
      */
    pub fn get_most_recent_note(&self, midi_channel: i32) -> MPENote {
        
        todo!();
        /*
            if (auto* note = getLastNotePlayedPtr (midiChannel))
            return *note;

        return {};
        */
    }
    
    /**
      | Returns the most recent note that is
      | not the note passed in. If there is no
      | such note, this returns an invalid MPENote
      | (check with note.isValid() before
      | use!).
      | 
      | This helper method might be useful for
      | some custom voice handling algorithms.
      |
      */
    pub fn get_most_recent_note_other_than(&self, other_than_this_note: MPENote) -> MPENote {
        
        todo!();
        /*
            for (auto i = notes.size(); --i >= 0;)
        {
            auto& note = notes.getReference (i);

            if (note != otherThanThisNote)
                return note;
        }

        return {};
        */
    }
    
    pub fn get_note_ptr(&self, 
        midi_channel:     i32,
        midi_note_number: i32) -> *const MPENote {
        
        todo!();
        /*
            for (int i = 0; i < notes.size(); ++i)
        {
            auto& note = notes.getReference (i);

            if (note.midiChannel == midiChannel && note.initialNote == midiNoteNumber)
                return &note;
        }

        return nullptr;
        */
    }
    
    pub fn get_note_ptr_mut(&mut self, 
        midi_channel:     i32,
        midi_note_number: i32) -> *mut MPENote {
        
        todo!();
        /*
            return const_cast<MPENote*> (static_cast<const MPEInstrument&> (*this).getNotePtr (midiChannel, midiNoteNumber));
        */
    }
    
    pub fn get_note_ptr_with_tracking_mode(&self, 
        midi_channel: i32,
        mode:         MpeInstrumentTrackingMode) -> *const MPENote {
        
        todo!();
        /*
            // for the "all notes" tracking mode, this method can never possibly
        // work because it returns 0 or 1 note but there might be more than one!
        jassert (mode != allNotesOnChannel);

        if (mode == lastNotePlayedOnChannel)  return getLastNotePlayedPtr (midiChannel);
        if (mode == lowestNoteOnChannel)      return getLowestNotePtr (midiChannel);
        if (mode == highestNoteOnChannel)     return getHighestNotePtr (midiChannel);

        return nullptr;
        */
    }
    
    pub fn get_note_ptr_with_tracking_mode_mut(&mut self, 
        midi_channel: i32,
        mode:         MpeInstrumentTrackingMode) -> *mut MPENote {
        
        todo!();
        /*
            return const_cast<MPENote*> (static_cast<const MPEInstrument&> (*this).getNotePtr (midiChannel, mode));
        */
    }
    
    pub fn get_last_note_played_ptr(&self, midi_channel: i32) -> *const MPENote {
        
        todo!();
        /*
            for (auto i = notes.size(); --i >= 0;)
        {
            auto& note = notes.getReference (i);

            if (note.midiChannel == midiChannel
                 && (note.keyState == MPENote::keyDown || note.keyState == MPENote::keyDownAndSustained))
                return &note;
        }

        return nullptr;
        */
    }
    
    pub fn get_last_note_played_ptr_mut(&mut self, midi_channel: i32) -> *mut MPENote {
        
        todo!();
        /*
            return const_cast<MPENote*> (static_cast<const MPEInstrument&> (*this).getLastNotePlayedPtr (midiChannel));
        */
    }
    
    pub fn get_highest_note_ptr(&self, midi_channel: i32) -> *const MPENote {
        
        todo!();
        /*
            int initialNoteMax = -1;
        const MPENote* result = nullptr;

        for (auto i = notes.size(); --i >= 0;)
        {
            auto& note = notes.getReference (i);

            if (note.midiChannel == midiChannel
                 && (note.keyState == MPENote::keyDown || note.keyState == MPENote::keyDownAndSustained)
                 && note.initialNote > initialNoteMax)
            {
                result = &note;
                initialNoteMax = note.initialNote;
            }
        }

        return result;
        */
    }
    
    pub fn get_highest_note_ptr_mut(&mut self, midi_channel: i32) -> *mut MPENote {
        
        todo!();
        /*
            return const_cast<MPENote*> (static_cast<const MPEInstrument&> (*this).getHighestNotePtr (midiChannel));
        */
    }
    
    pub fn get_lowest_note_ptr(&self, midi_channel: i32) -> *const MPENote {
        
        todo!();
        /*
            int initialNoteMin = 128;
        const MPENote* result = nullptr;

        for (auto i = notes.size(); --i >= 0;)
        {
            auto& note = notes.getReference (i);

            if (note.midiChannel == midiChannel
                 && (note.keyState == MPENote::keyDown || note.keyState == MPENote::keyDownAndSustained)
                 && note.initialNote < initialNoteMin)
            {
                result = &note;
                initialNoteMin = note.initialNote;
            }
        }

        return result;
        */
    }
    
    pub fn get_lowest_note_ptr_mut(&mut self, midi_channel: i32) -> *mut MPENote {
        
        todo!();
        /*
            return const_cast<MPENote*> (static_cast<const MPEInstrument&> (*this).getLowestNotePtr (midiChannel));
        */
    }
    
    /**
      | Discard all currently playing notes.
      | 
      | This will also call the noteReleased
      | listener callback for all of them.
      |
      */
    pub fn release_all_notes(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (auto i = notes.size(); --i >= 0;)
        {
            auto& note = notes.getReference (i);
            note.keyState = MPENote::off;
            note.noteOffVelocity = MPEValue::from7BitInt (64); // some reasonable number
            listeners.call ([&] (MpeInstrumentListener& l) { l.noteReleased (note); });
        }

        notes.clear();
        */
    }
}
