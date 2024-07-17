// we didnt have breakfast!
//
// people invented breakfast to sell you cereal!
//
//- M. Saylor
//
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPEUtils.h]

/**
  | This class handles the assignment of
  | new MIDI notes to member channels of
  | an active MPE zone.
  | 
  | To use it, create an instance passing
  | in the MPE zone that it should operate
  | on and then call use the findMidiChannelForNewNote()
  | method for all note-on messages and
  | the noteOff() method for all note-off
  | messages.
  | 
  | @tags{Audio}
  |
  */
pub struct MPEChannelAssigner {
    is_legacy:                  bool, // default = false
    zone:                       Box<MpeZoneLayoutZone>,
    channel_increment:          i32,
    num_channels:               i32,
    first_channel:              i32,
    last_channel:               i32,
    midi_channel_last_assigned: i32,
    midi_channels:              [MpeChannelAssignerMidiChannel; 17],
}

impl From<MpeZoneLayoutZone> for MPEChannelAssigner {

    fn from(zone_to_use: MpeZoneLayoutZone) -> Self {
    
        todo!();
        /*


            : zone                    (new MPEZoneLayout::Zone (zoneToUse)),
          channelIncrement        (zone->isLowerZone() ? 1 : -1),
          numChannels             (zone->numMemberChannels),
          firstChannel            (zone->getFirstMemberChannel()),
          lastChannel             (zone->getLastMemberChannel()),
          midiChannelLastAssigned (firstChannel - channelIncrement)

        // must be an active MPE zone!
        jassert (numChannels > 0);
        */
    }
}
    
impl From<Option<Range<i32>>> for MPEChannelAssigner {

    /**
      | Legacy mode constructor.
      | 
      | This will assign channels within the
      | specified range.
      |
      */
    fn from(channel_range: Option<Range<i32>>) -> Self {

        let channel_range = channel_range.unwrap_or(1..17);
    
        todo!();
        /*


            : isLegacy                (true),
          channelIncrement        (1),
          numChannels             (channelRange.getLength()),
          firstChannel            (channelRange.getStart()),
          lastChannel             (channelRange.getEnd() - 1),
          midiChannelLastAssigned (firstChannel - channelIncrement)

        // must have at least one channel!
        jassert (! channelRange.isEmpty());
        */
    }
}

impl MPEChannelAssigner {

    /**
      | Constructor.
      | 
      | This will assign channels within the
      | range of the specified MPE zone.
      |
      */
    pub fn from_remapped_zone(zone_to_remap: MpeZoneLayoutZone) -> Self {
    
        todo!();
        /*


            : zone             (zoneToRemap),
          channelIncrement (zone.isLowerZone() ? 1 : -1),
          firstChannel     (zone.getFirstMemberChannel()),
          lastChannel      (zone.getLastMemberChannel())

        // must be an active MPE zone!
        jassert (zone.numMemberChannels > 0);
        zeroArrays();
        */
    }
    
    /**
      | This method will use a set of rules recommended
      | in the MPE specification to determine
      | which member channel the specified
      | MIDI note should be assigned to and will
      | return this channel number.
      | 
      | The rules have the following precedence:
      | 
      | - find a free channel on which the last
      | note played was the same as the one specified
      | 
      | - find the next free channel in round-robin
      | assignment
      | 
      | - find the channel number that is currently
      | playing the closest note (but not the
      | same)
      | 
      | -----------
      | @param noteNumber
      | 
      | the MIDI note number to be assigned to
      | a channel
      | 
      | -----------
      | @return
      | 
      | the zone's member channel that this
      | note should be assigned to
      |
      */
    pub fn find_midi_channel_for_new_note(&mut self, note_number: i32) -> i32 {
        
        todo!();
        /*
            if (numChannels <= 1)
            return firstChannel;

        for (auto ch = firstChannel; (isLegacy || zone->isLowerZone() ? ch <= lastChannel : ch >= lastChannel); ch += channelIncrement)
        {
            if (midiChannels[ch].isFree() && midiChannels[ch].lastNotePlayed == noteNumber)
            {
                midiChannelLastAssigned = ch;
                midiChannels[ch].notes.add (noteNumber);
                return ch;
            }
        }

        for (auto ch = midiChannelLastAssigned + channelIncrement; ; ch += channelIncrement)
        {
            if (ch == lastChannel + channelIncrement)  // loop wrap-around
                ch = firstChannel;

            if (midiChannels[ch].isFree())
            {
                midiChannelLastAssigned = ch;
                midiChannels[ch].notes.add (noteNumber);
                return ch;
            }

            if (ch == midiChannelLastAssigned)
                break; // no free channels!
        }

        midiChannelLastAssigned = findMidiChannelPlayingClosestNonequalNote (noteNumber);
        midiChannels[midiChannelLastAssigned].notes.add (noteNumber);

        return midiChannelLastAssigned;
        */
    }
    
    /**
      | You must call this method for all note-offs
      | that you receive so that this class can
      | keep track of the currently playing
      | notes internally.
      | 
      | You can specify the channel number the
      | note off happened on. If you don't, it
      | will look through all channels to find
      | the registered midi note matching the
      | given note number.
      |
      */
    pub fn note_off(
        &mut self, 
        note_number:  i32,
        midi_channel: Option<i32>

    ) {

        let midi_channel: i32 = midi_channel.unwrap_or(-1);
        
        todo!();
        /*
            const auto removeNote = [] (MpeChannelAssignerMidiChannel& ch, int noteNum)
        {
            if (ch.notes.removeAllInstancesOf (noteNum) > 0)
            {
                ch.lastNotePlayed = noteNum;
                return true;
            }

            return false;
        };

        if (midiChannel >= 0 && midiChannel <= 16)
        {
            removeNote (midiChannels[midiChannel], noteNumber);
            return;
        }

        for (auto& ch : midiChannels)
        {
            if (removeNote (ch, noteNumber))
                return;
        }
        */
    }
    
    /**
      | Call this to clear all currently playing
      | notes.
      |
      */
    pub fn all_notes_off(&mut self)  {
        
        todo!();
        /*
            for (auto& ch : midiChannels)
        {
            if (ch.notes.size() > 0)
                ch.lastNotePlayed = ch.notes.getLast();

            ch.notes.clear();
        }
        */
    }
    
    pub fn find_midi_channel_playing_closest_nonequal_note(&mut self, note_number: i32) -> i32 {
        
        todo!();
        /*
            auto channelWithClosestNote = firstChannel;
        int closestNoteDistance = 127;

        for (auto ch = firstChannel; (isLegacy || zone->isLowerZone() ? ch <= lastChannel : ch >= lastChannel); ch += channelIncrement)
        {
            for (auto note : midiChannels[ch].notes)
            {
                auto noteDistance = std::abs (note - noteNumber);

                if (noteDistance > 0 && noteDistance < closestNoteDistance)
                {
                    closestNoteDistance = noteDistance;
                    channelWithClosestNote = ch;
                }
            }
        }

        return channelWithClosestNote;
        */
    }
    
    
    pub fn remap_midi_channel_if_needed(&mut self, 
        message:      &mut MidiMessage,
        mpe_sourceid: u32)  {
        
        todo!();
        /*
            auto channel = message.getChannel();

        if (! zone.isUsingChannelAsMemberChannel (channel))
            return;

        if (channel == zone.getMasterChannel() && (message.isResetAllControllers() || message.isAllNotesOff()))
        {
            clearSource (mpeSourceID);
            return;
        }

        auto sourceAndChannelID = (((uint32) mpeSourceID << 5) | (uint32) (channel));

        if (messageIsNoteData (message))
        {
            ++counter;

            // fast path - no remap
            if (applyRemapIfExisting (channel, sourceAndChannelID, message))
                return;

            // find existing remap
            for (int chan = firstChannel; (zone.isLowerZone() ? chan <= lastChannel : chan >= lastChannel); chan += channelIncrement)
                if (applyRemapIfExisting (chan, sourceAndChannelID, message))
                    return;

            // no remap necessary
            if (sourceAndChannel[channel] == notMPE)
            {
                lastUsed[channel] = counter;
                sourceAndChannel[channel] = sourceAndChannelID;
                return;
            }

            // remap source & channel to new channel
            auto chan = getBestChanToReuse();

            sourceAndChannel[chan] = sourceAndChannelID;
            lastUsed[chan] = counter;
            message.setChannel (chan);
        }
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (auto& s : sourceAndChannel)
            s = notMPE;
        */
    }
    
    pub fn clear_channel(&mut self, channel: i32)  {
        
        todo!();
        /*
            sourceAndChannel[channel] = notMPE;
        */
    }
    
    pub fn clear_source(&mut self, mpe_sourceid: u32)  {
        
        todo!();
        /*
            for (auto& s : sourceAndChannel)
        {
            if (uint32 (s >> 5) == mpeSourceID)
            {
                s = notMPE;
                return;
            }
        }
        */
    }
    
    pub fn apply_remap_if_existing(&mut self, 
        channel:              i32,
        source_and_channelid: u32,
        m:                    &mut MidiMessage) -> bool {
        
        todo!();
        /*
            if (sourceAndChannel[channel] == sourceAndChannelID)
        {
            if (m.isNoteOff())
                sourceAndChannel[channel] = notMPE;
            else
                lastUsed[channel] = counter;

            m.setChannel (channel);
            return true;
        }

        return false;
        */
    }
    
    pub fn get_best_chan_to_reuse(&self) -> i32 {
        
        todo!();
        /*
            for (int chan = firstChannel; (zone.isLowerZone() ? chan <= lastChannel : chan >= lastChannel); chan += channelIncrement)
            if (sourceAndChannel[chan] == notMPE)
                return chan;

        auto bestChan = firstChannel;
        auto bestLastUse = counter;

        for (int chan = firstChannel; (zone.isLowerZone() ? chan <= lastChannel : chan >= lastChannel); chan += channelIncrement)
        {
            if (lastUsed[chan] < bestLastUse)
            {
                bestLastUse = lastUsed[chan];
                bestChan = chan;
            }
        }

        return bestChan;
        */
    }
    
    pub fn zero_arrays(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < 17; ++i)
        {
            sourceAndChannel[i] = 0;
            lastUsed[i] = 0;
        }
        */
    }
}
