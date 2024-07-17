crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEZoneLayout.h]

/**
  | This class represents the current MPE zone
  | layout of a device capable of handling MPE.
  |
  | An MPE device can have up to two zones:
  | a lower zone with master channel 1 and
  | allocated MIDI channels increasing from
  | channel 2, and an upper zone with master
  | channel 16 and allocated MIDI channels
  | decreasing from channel 15. MPE mode is
  | enabled on a device when one of these zones is
  | active and disabled when both are inactive.
  |
  | Use the MPEMessages helper class to convert
  | the zone layout represented by this object to
  | MIDI message sequences that you can send to an
  | Expressive MIDI device to set its zone layout,
  | add zones etc.
  |
  | @see MPEInstrument
  |
  | @tags{Audio}
  */
pub struct MPEZoneLayout {
    lower_zone:   MpeZoneLayoutZone, // { true, 0 };
    upper_zone:   MpeZoneLayoutZone, // { false, 0 };
    rpn_detector: MidiRPNDetector,
    listeners:    ListenerList<Box<dyn MpeZoneLayoutListener>>,
}

impl Default for MPEZoneLayout {
    
    /**
      | Default constructor.
      | 
      | This will create a layout with inactive
      | lower and upper zones, representing
      | a device with MPE mode disabled.
      | 
      | You can set the lower or upper MPE zones
      | using the setZone() method.
      | 
      | @see setZone
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEZoneLayout.cpp]
impl MPEZoneLayout {

    /**
      | Returns a struct representing the lower
      | MPE zone.
      |
      */
    pub fn get_lower_zone(&self) -> MpeZoneLayoutZone {
        
        todo!();
        /*
            return lowerZone;
        */
    }

    /**
      | Returns a struct representing the upper
      | MPE zone.
      |
      */
    pub fn get_upper_zone(&self) -> MpeZoneLayoutZone {
        
        todo!();
        /*
            return upperZone;
        */
    }

    /**
      | Copy constuctor. This will not copy
      | the listeners registered to the MPEZoneLayout.
      |
      */
    pub fn new(other: &MPEZoneLayout) -> Self {
    
        todo!();
        /*


            : lowerZone (other.lowerZone),
          upperZone (other.upperZone)
        */
    }
    
    /**
      | Copy assignment operator. This will
      | not copy the listeners registered to
      | the MPEZoneLayout.
      |
      */
    pub fn assign_from(&mut self, other: &MPEZoneLayout) -> &mut MPEZoneLayout {
        
        todo!();
        /*
            lowerZone = other.lowerZone;
        upperZone = other.upperZone;

        sendLayoutChangeMessage();

        return *this;
        */
    }
    
    pub fn send_layout_change_message(&mut self)  {
        
        todo!();
        /*
            listeners.call ([this] (MpeZoneLayoutListener& l) { l.zoneLayoutChanged (*this); });
        */
    }
    
    pub fn set_zone(&mut self, 
        is_lower:                 bool,
        num_member_channels:      i32,
        per_note_pitchbend_range: i32,
        master_pitchbend_range:   i32)  {
        
        todo!();
        /*
            checkAndLimitZoneParameters (0, 15,  numMemberChannels);
        checkAndLimitZoneParameters (0, 96,  perNotePitchbendRange);
        checkAndLimitZoneParameters (0, 96,  masterPitchbendRange);

        if (isLower)
            lowerZone = { true, numMemberChannels, perNotePitchbendRange, masterPitchbendRange };
        else
            upperZone = { false, numMemberChannels, perNotePitchbendRange, masterPitchbendRange };

        if (numMemberChannels > 0)
        {
            auto totalChannels = lowerZone.numMemberChannels + upperZone.numMemberChannels;

            if (totalChannels >= 15)
            {
                if (isLower)
                    upperZone.numMemberChannels = 14 - numMemberChannels;
                else
                    lowerZone.numMemberChannels = 14 - numMemberChannels;
            }
        }

        sendLayoutChangeMessage();
        */
    }
    
    /**
      | Sets the lower zone of this layout.
      |
      */
    pub fn set_lower_zone(
        &mut self, 
        num_member_channels:      Option<i32>,
        per_note_pitchbend_range: Option<i32>,
        master_pitchbend_range:   Option<i32>

    ) {

        let num_member_channels:      i32 = num_member_channels.unwrap_or(0);
        let per_note_pitchbend_range: i32 = per_note_pitchbend_range.unwrap_or(48);
        let master_pitchbend_range:   i32 = master_pitchbend_range.unwrap_or(2);
        
        todo!();
        /*
            setZone (true, numMemberChannels, perNotePitchbendRange, masterPitchbendRange);
        */
    }
    
    /**
      | Sets the upper zone of this layout.
      |
      */
    pub fn set_upper_zone(
        &mut self, 
        num_member_channels:      Option<i32>,
        per_note_pitchbend_range: Option<i32>,
        master_pitchbend_range:   Option<i32>

    ) {

        let num_member_channels:      i32 = num_member_channels.unwrap_or(0);
        let per_note_pitchbend_range: i32 = per_note_pitchbend_range.unwrap_or(48);
        let master_pitchbend_range:   i32 = master_pitchbend_range.unwrap_or(2);
        
        todo!();
        /*
            setZone (false, numMemberChannels, perNotePitchbendRange, masterPitchbendRange);
        */
    }
    
    /**
      | Clears the lower and upper zones of this
      | layout, making them both inactive and
      | disabling MPE mode.
      |
      */
    pub fn clear_all_zones(&mut self)  {
        
        todo!();
        /*
            lowerZone = { true, 0 };
        upperZone = { false, 0 };

        sendLayoutChangeMessage();
        */
    }
    
    /**
      | Pass incoming MIDI messages to an object
      | of this class if you want the zone layout
      | to properly react to MPE RPN messages
      | like an MPE device.
      | 
      | MPEMessages::rpnNumber will add or
      | remove zones; RPN 0 will set the per-note
      | or master pitchbend ranges.
      | 
      | Any other MIDI messages will be ignored
      | by this class.
      | 
      | @see MPEMessages
      |
      */
    pub fn process_next_midi_event(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            if (! message.isController())
            return;

        MidiRPNMessage rpn;

        if (rpnDetector.parseControllerMessage (message.getChannel(),
                                                message.getControllerNumber(),
                                                message.getControllerValue(),
                                                rpn))
        {
            processRpnMessage (rpn);
        }
        */
    }
    
    pub fn process_rpn_message(&mut self, rpn: MidiRPNMessage)  {
        
        todo!();
        /*
            if (rpn.parameterNumber == MPEMessages::zoneLayoutMessagesRpnNumber)
            processZoneLayoutRpnMessage (rpn);
        else if (rpn.parameterNumber == 0)
            processPitchbendRangeRpnMessage (rpn);
        */
    }
    
    pub fn process_zone_layout_rpn_message(&mut self, rpn: MidiRPNMessage)  {
        
        todo!();
        /*
            if (rpn.value < 16)
        {
            if (rpn.channel == 1)
                setLowerZone (rpn.value);
            else if (rpn.channel == 16)
                setUpperZone (rpn.value);
        }
        */
    }
    
    pub fn update_master_pitchbend(&mut self, 
        zone:  &mut MpeZoneLayoutZone,
        value: i32)  {
        
        todo!();
        /*
            if (zone.masterPitchbendRange != value)
        {
            checkAndLimitZoneParameters (0, 96, zone.masterPitchbendRange);
            zone.masterPitchbendRange = value;
            sendLayoutChangeMessage();
        }
        */
    }
    
    pub fn update_per_note_pitchbend_range(&mut self, 
        zone:  &mut MpeZoneLayoutZone,
        value: i32)  {
        
        todo!();
        /*
            if (zone.perNotePitchbendRange != value)
        {
            checkAndLimitZoneParameters (0, 96, zone.perNotePitchbendRange);
            zone.perNotePitchbendRange = value;
            sendLayoutChangeMessage();
        }
        */
    }
    
    pub fn process_pitchbend_range_rpn_message(&mut self, rpn: MidiRPNMessage)  {
        
        todo!();
        /*
            if (rpn.channel == 1)
        {
            updateMasterPitchbend (lowerZone, rpn.value);
        }
        else if (rpn.channel == 16)
        {
            updateMasterPitchbend (upperZone, rpn.value);
        }
        else
        {
            if (lowerZone.isUsingChannelAsMemberChannel (rpn.channel))
                updatePerNotePitchbendRange (lowerZone, rpn.value);
            else if (upperZone.isUsingChannelAsMemberChannel (rpn.channel))
                updatePerNotePitchbendRange (upperZone, rpn.value);
        }
        */
    }
    
    /**
      | Pass incoming MIDI buffers to an object
      | of this class if you want the zone layout
      | to properly react to MPE RPN messages
      | like an MPE device.
      | 
      | MPEMessages::rpnNumber will add or
      | remove zones; RPN 0 will set the per-note
      | or master pitchbend ranges.
      | 
      | Any other MIDI messages will be ignored
      | by this class.
      | 
      | @see MPEMessages
      |
      */
    pub fn process_next_midi_buffer(&mut self, buffer: &MidiBuffer)  {
        
        todo!();
        /*
            for (const auto metadata : buffer)
            processNextMidiEvent (metadata.getMessage());
        */
    }
    
    /**
      | Adds a listener.
      |
      */
    pub fn add_listener(&mut self, listener_to_add: *mut dyn MpeZoneLayoutListener)  {
        
        todo!();
        /*
            listeners.add (listenerToAdd);
        */
    }
    
    /**
      | Removes a listener.
      |
      */
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn MpeZoneLayoutListener)  {
        
        todo!();
        /*
            listeners.remove (listenerToRemove);
        */
    }
    
    pub fn check_and_limit_zone_parameters(&mut self, 
        min_value:                i32,
        max_value:                i32,
        value_to_check_and_limit: &mut i32)  {
        
        todo!();
        /*
            if (valueToCheckAndLimit < minValue || valueToCheckAndLimit > maxValue)
        {
            // if you hit this, one of the parameters you supplied for this zone
            // was not within the allowed range!
            // we fit this back into the allowed range here to maintain a valid
            // state for the zone, but probably the resulting zone is not what you
            // wanted it to be!
            jassertfalse;

            valueToCheckAndLimit = jlimit (minValue, maxValue, valueToCheckAndLimit);
        }
        */
    }
}
