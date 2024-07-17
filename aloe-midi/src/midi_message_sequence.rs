crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiMessageSequence.h]

/**
  | A sequence of timestamped midi messages.
  |
  | This allows the sequence to be manipulated,
  | and also to be read from and written to
  | a standard midi file.
  |
  | @see MidiMessage, MidiFile
  |
  | @tags{Audio}
  */
#[derive(Default)] // Creates an empty midi sequence object
#[leak_detector]
pub struct MidiMessageSequence {

    list: Vec<Box<MidiMessageSequenceMidiEventHolder>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiMessageSequence.cpp]
impl MidiMessageSequence {

    /**
      | Creates a copy of another sequence.
      |
      */
    pub fn new_from_other_ref(other: &MidiMessageSequence) -> Self {
    
        todo!();
        /*
        list.addCopiesOf (other.list);

        for (int i = 0; i < list.size(); ++i)
        {
            auto noteOffIndex = other.getIndexOfMatchingKeyUp (i);

            if (noteOffIndex >= 0)
                list.getUnchecked(i)->noteOffObject = list.getUnchecked (noteOffIndex);
        }
        */
    }
    
    /**
      | Replaces this sequence with another
      | one.
      |
      */
    pub fn assign_from_other_ref(&mut self, other: &MidiMessageSequence) -> &mut MidiMessageSequence {
        
        todo!();
        /*
            MidiMessageSequence otherCopy (other);
        swapWith (otherCopy);
        return *this;
        */
    }
    
    pub fn new_from_other(other: MidiMessageSequence) -> Self {
    
        todo!();
        /*
        : list(std::move (other.list)),

        
        */
    }
    
    pub fn assign_from(&mut self, other: MidiMessageSequence) -> &mut MidiMessageSequence {
        
        todo!();
        /*
            list = std::move (other.list);
        return *this;
        */
    }
    
    /**
      | Swaps this sequence with another one.
      |
      */
    pub fn swap_with(&mut self, other: &mut MidiMessageSequence)  {
        
        todo!();
        /*
            list.swapWith (other.list);
        */
    }
    
    /**
      | Clears the sequence.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            list.clear();
        */
    }
    
    /**
      | Returns the number of events in the sequence.
      |
      */
    pub fn get_num_events(&self) -> i32 {
        
        todo!();
        /*
            return list.size();
        */
    }
    
    /**
      | Returns a pointer to one of the events.
      |
      */
    pub fn get_event_pointer(&self, index: i32) -> *mut MidiMessageSequenceMidiEventHolder {
        
        todo!();
        /*
            return list[index];
        */
    }
    
    /**
      | Iterator for the list of MidiEventHolders
      |
      */
    pub fn begin_mut(&mut self) -> *mut *mut MidiMessageSequenceMidiEventHolder {
        
        todo!();
        /*
            return list.begin();
        */
    }
    
    /**
      | Iterator for the list of MidiEventHolders
      |
      */
    pub fn begin(&self) -> *const *const MidiMessageSequenceMidiEventHolder {
        
        todo!();
        /*
            return list.begin();
        */
    }
    
    /**
      | Iterator for the list of MidiEventHolders
      |
      */
    pub fn end_mut(&mut self) -> *mut *mut MidiMessageSequenceMidiEventHolder {
        
        todo!();
        /*
            return list.end();
        */
    }
    
    /**
      | Iterator for the list of MidiEventHolders
      |
      */
    pub fn end(&self) -> *const *const MidiMessageSequenceMidiEventHolder {
        
        todo!();
        /*
            return list.end();
        */
    }
    
    /**
      | Returns the time of the note-up that
      | matches the note-on at this index. If
      | the event at this index isn't a note-on,
      | it'll just return 0. 
      | @see MidiMessageSequence::MidiMessageSequenceMidiEventHolder::noteOffObject
      |
      */
    pub fn get_time_of_matching_key_up(&self, index: i32) -> f64 {
        
        todo!();
        /*
            if (auto* meh = list[index])
            if (auto* noteOff = meh->noteOffObject)
                return noteOff->message.getTimeStamp();

        return 0;
        */
    }
    
    /**
      | Returns the index of the note-up that
      | matches the note-on at this index. If
      | the event at this index isn't a note-on,
      | it'll just return -1. @see MidiMessageSequence::MidiMessageSequenceMidiEventHolder::noteOffObject
      |
      */
    pub fn get_index_of_matching_key_up(&self, index: i32) -> i32 {
        
        todo!();
        /*
            if (auto* meh = list[index])
        {
            if (auto* noteOff = meh->noteOffObject)
            {
                for (int i = index; i < list.size(); ++i)
                    if (list.getUnchecked(i) == noteOff)
                        return i;

                jassertfalse; // we've somehow got a pointer to a note-off object that isn't in the sequence
            }
        }

        return -1;
        */
    }
    
    /**
      | Returns the index of an event.
      |
      */
    pub fn get_index_of(&self, event: *const MidiMessageSequenceMidiEventHolder) -> i32 {
        
        todo!();
        /*
            return list.indexOf (event);
        */
    }
    
    /**
      | Returns the index of the first event
      | on or after the given timestamp. If the
      | time is beyond the end of the sequence,
      | this will return the number of events.
      */
    pub fn get_next_index_at_time(&self, time_stamp: f64) -> i32 {
        
        todo!();
        /*
            auto numEvents = list.size();
        int i;

        for (i = 0; i < numEvents; ++i)
            if (list.getUnchecked(i)->message.getTimeStamp() >= timeStamp)
                break;

        return i;
        */
    }
    
    /**
      | Returns the timestamp of the first event
      | in the sequence. @see getEndTime
      */
    pub fn get_start_time(&self) -> f64 {
        
        todo!();
        /*
            return getEventTime (0);
        */
    }
    
    /**
      | Returns the timestamp of the last event
      | in the sequence. @see getStartTime
      |
      */
    pub fn get_end_time(&self) -> f64 {
        
        todo!();
        /*
            return getEventTime (list.size() - 1);
        */
    }
    
    /**
      | Returns the timestamp of the event at
      | a given index. If the index is out-of-range,
      | this will return 0.0
      |
      */
    pub fn get_event_time(&self, index: i32) -> f64 {
        
        todo!();
        /*
            if (auto* meh = list[index])
            return meh->message.getTimeStamp();

        return 0;
        */
    }
    
    /**
      | Inserts a midi message into the sequence.
      | 
      | -----------
      | @note
      | 
      | The index at which the new message gets
      | inserted will depend on its timestamp,
      | 
      | because the sequence is kept sorted.
      | ----------
      | @note
      | 
      | Remember to call updateMatchedPairs()
      | after adding note-on events.
      | 
      | -----------
      | @param newMessage
      | 
      | the new message to add (an internal copy
      | will be made)
      | ----------
      | @param timeAdjustment
      | 
      | an optional value to add to the timestamp
      | of the message
      | 
      | that will be inserted
      | 
      | @see updateMatchedPairs
      |
      */
    pub fn add_event(
        &mut self, 
        new_event:       *mut MidiMessageSequenceMidiEventHolder,
        time_adjustment: Option<f64>

    ) -> *mut MidiMessageSequenceMidiEventHolder {

        let time_adjustment: f64 = time_adjustment.unwrap_or(0.0);
        
        todo!();
        /*
            newEvent->message.addToTimeStamp (timeAdjustment);
        auto time = newEvent->message.getTimeStamp();
        int i;

        for (i = list.size(); --i >= 0;)
            if (list.getUnchecked(i)->message.getTimeStamp() <= time)
                break;

        list.insert (i + 1, newEvent);
        return newEvent;
        */
    }
    
    /**
      | Inserts a midi message into the sequence.
      | 
      | -----------
      | @note
      | 
      | The index at which the new message gets
      | inserted will depend on its timestamp,
      | because the sequence is kept sorted.
      | ----------
      | @note
      | 
      | Remember to call updateMatchedPairs()
      | after adding note-on events.
      | 
      | -----------
      | @param newMessage
      | 
      | the new message to add (an internal copy
      | will be made)
      | ----------
      | @param timeAdjustment
      | 
      | an optional value to add to the timestamp
      | of the message that will be inserted
      | 
      | @see updateMatchedPairs
      |
      */
    pub fn add_event_from_other_ref(
        &mut self, 
        new_message:     &MidiMessage,
        time_adjustment: Option<f64>

    ) -> *mut MidiMessageSequenceMidiEventHolder {

        let time_adjustment: f64 = time_adjustment.unwrap_or(0.0);
        
        todo!();
        /*
            return addEvent (new MidiMessageSequenceMidiEventHolder (newMessage), timeAdjustment);
        */
    }
    
    pub fn add_event_from_new_message(
        &mut self, 
        new_message:     MidiMessage,
        time_adjustment: f64

    ) -> *mut MidiMessageSequenceMidiEventHolder {
        
        todo!();
        /*
            return addEvent (new MidiMessageSequenceMidiEventHolder (std::move (newMessage)), timeAdjustment);
        */
    }
    
    /**
      | Deletes one of the events in the sequence.
      | 
      | -----------
      | @note
      | 
      | Remember to call updateMatchedPairs()
      | after removing events.
      | 
      | -----------
      | @param index
      | 
      | the index of the event to delete
      | ----------
      | @param deleteMatchingNoteUp
      | 
      | whether to also remove the matching
      | note-off if the event you're removing
      | is a note-on
      |
      */
    pub fn delete_event(&mut self, 
        index:                   i32,
        delete_matching_note_up: bool)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, list.size()))
        {
            if (deleteMatchingNoteUp)
                deleteEvent (getIndexOfMatchingKeyUp (index), false);

            list.remove (index);
        }
        */
    }
    
    /**
      | Merges another sequence into this one.
      | 
      | -----------
      | @note
      | 
      | Remember to call updateMatchedPairs()
      | after using this method.
      | 
      | -----------
      | @param other
      | 
      | the sequence to add from
      | ----------
      | @param timeAdjustmentDelta
      | 
      | an amount to add to the timestamps of
      | the midi events as they are read from
      | the other sequence
      |
      */
    pub fn add_sequence(&mut self, 
        other:           &MidiMessageSequence,
        time_adjustment: f64)  {
        
        todo!();
        /*
            for (auto* m : other)
        {
            auto newOne = new MidiMessageSequenceMidiEventHolder (m->message);
            newOne->message.addToTimeStamp (timeAdjustment);
            list.add (newOne);
        }

        sort();
        */
    }
    
    /**
      | Merges another sequence into this one.
      | 
      | -----------
      | @note
      | 
      | Remember to call updateMatchedPairs()
      | after using this method.
      | 
      | -----------
      | @param other
      | 
      | the sequence to add from
      | ----------
      | @param timeAdjustmentDelta
      | 
      | an amount to add to the timestamps of
      | the midi events as they are read from
      | the other sequence
      | ----------
      | @param firstAllowableDestTime
      | 
      | events will not be added if their time
      | is earlier than this time. (This is after
      | their time has been adjusted by the timeAdjustmentDelta)
      | ----------
      | @param endOfAllowableDestTimes
      | 
      | events will not be added if their time
      | is equal to or greater than this time.
      | (This is after their time has been adjusted
      | by the timeAdjustmentDelta)
      |
      */
    pub fn add_sequence_with_allowable_time_info(
        &mut self, 
        other:                       &MidiMessageSequence,
        time_adjustment:             f64,
        first_allowable_time:        f64,
        end_of_allowable_dest_times: f64)  {
        
        todo!();
        /*
            for (auto* m : other)
        {
            auto t = m->message.getTimeStamp() + timeAdjustment;

            if (t >= firstAllowableTime && t < endOfAllowableDestTimes)
            {
                auto newOne = new MidiMessageSequenceMidiEventHolder (m->message);
                newOne->message.setTimeStamp (t);
                list.add (newOne);
            }
        }

        sort();
        */
    }
    
    /**
      | Forces a sort of the sequence. You may
      | need to call this if you've manually
      | modified the timestamps of some events
      | such that the overall order now needs
      | updating.
      |
      */
    pub fn sort(&mut self)  {
        
        todo!();
        /*
            std::stable_sort (list.begin(), list.end(),
                          [] (const MidiMessageSequenceMidiEventHolder* a, const MidiMessageSequenceMidiEventHolder* b) { return a->message.getTimeStamp() < b->message.getTimeStamp(); });
        */
    }
    
    /**
      | Makes sure all the note-on and note-off
      | pairs are up-to-date.
      | 
      | -----------
      | @note
      | 
      | Call this after re-ordering messages
      | or deleting/adding messages, and it
      | 
      | will scan the list and make sure all the
      | note-offs in the MidiMessageSequenceMidiEventHolder
      | 
      | structures are pointing at the correct
      | ones.
      |
      */
    pub fn update_matched_pairs(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < list.size(); ++i)
        {
            auto* meh = list.getUnchecked(i);
            auto& m1 = meh->message;

            if (m1.isNoteOn())
            {
                meh->noteOffObject = nullptr;
                auto note = m1.getNoteNumber();
                auto chan = m1.getChannel();
                auto len = list.size();

                for (int j = i + 1; j < len; ++j)
                {
                    auto* meh2 = list.getUnchecked(j);
                    auto& m = meh2->message;

                    if (m.getNoteNumber() == note && m.getChannel() == chan)
                    {
                        if (m.isNoteOff())
                        {
                            meh->noteOffObject = meh2;
                            break;
                        }

                        if (m.isNoteOn())
                        {
                            auto newEvent = new MidiMessageSequenceMidiEventHolder (MidiMessage::noteOff (chan, note));
                            list.insert (j, newEvent);
                            newEvent->message.setTimeStamp (m.getTimeStamp());
                            meh->noteOffObject = newEvent;
                            break;
                        }
                    }
                }
            }
        }
        */
    }
    
    /**
      | Adds an offset to the timestamps of all
      | events in the sequence.
      | 
      | -----------
      | @param deltaTime
      | 
      | the amount to add to each timestamp.
      |
      */
    pub fn add_time_to_messages(&mut self, delta: f64)  {
        
        todo!();
        /*
            if (delta != 0)
            for (auto* m : list)
                m->message.addToTimeStamp (delta);
        */
    }
    
    /**
      | Copies all the messages for a particular
      | midi channel to another sequence.
      | 
      | -----------
      | @param channelNumberToExtract
      | 
      | the midi channel to look for, in the range
      | 1 to 16
      | ----------
      | @param destSequence
      | 
      | the sequence that the chosen events
      | should be copied to
      | ----------
      | @param alsoIncludeMetaEvents
      | 
      | if true, any meta-events (which don't
      | apply to a specific channel) will also
      | be copied across.
      | 
      | @see extractSysExMessages
      |
      */
    pub fn extract_midi_channel_messages(&self, 
        channel_number_to_extract: i32,
        dest_sequence:             &mut MidiMessageSequence,
        also_include_meta_events:  bool)  {
        
        todo!();
        /*
            for (auto* meh : list)
            if (meh->message.isForChannel (channelNumberToExtract)
                 || (alsoIncludeMetaEvents && meh->message.isMetaEvent()))
                destSequence.addEvent (meh->message);
        */
    }
    
    /**
      | Copies all midi sys-ex messages to another
      | sequence.
      | 
      | -----------
      | @param destSequence
      | 
      | this is the sequence to which any sys-exes
      | in this sequence will be added
      | 
      | @see extractMidiChannelMessages
      |
      */
    pub fn extract_sys_ex_messages(&self, dest_sequence: &mut MidiMessageSequence)  {
        
        todo!();
        /*
            for (auto* meh : list)
            if (meh->message.isSysEx())
                destSequence.addEvent (meh->message);
        */
    }
    
    /**
      | Removes any messages in this sequence
      | that have a specific midi channel.
      | 
      | -----------
      | @param channelNumberToRemove
      | 
      | the midi channel to look for, in the range
      | 1 to 16
      |
      */
    pub fn delete_midi_channel_messages(&mut self, channel_number_to_remove: i32)  {
        
        todo!();
        /*
            for (int i = list.size(); --i >= 0;)
            if (list.getUnchecked(i)->message.isForChannel (channelNumberToRemove))
                list.remove(i);
        */
    }
    
    /**
      | Removes any sys-ex messages from this
      | sequence.
      |
      */
    pub fn delete_sys_ex_messages(&mut self)  {
        
        todo!();
        /*
            for (int i = list.size(); --i >= 0;)
            if (list.getUnchecked(i)->message.isSysEx())
                list.remove(i);
        */
    }
    
    /**
      | Scans through the sequence to determine
      | the state of any midi controllers at
      | a given time.
      | 
      | -----------
      | @note
      | 
      | This will create a sequence of midi controller
      | changes that can be
      | 
      | used to set all midi controllers to the
      | state they would be in at the
      | 
      | specified time within this sequence.
      | ----------
      | @note
      | 
      | As well as controllers, it will also
      | recreate the midi program number
      | 
      | and pitch bend position.
      | 
      | -----------
      | @param channelNumber
      | 
      | the midi channel to look for, in the range
      | 1 to 16. Controllers for other channels
      | will be ignored.
      | ----------
      | @param time
      | 
      | the time at which you want to find out
      | the state - there are no explicit units
      | for this time measurement, it's the
      | same units as used for the timestamps
      | of the messages
      | ----------
      | @param resultMessages
      | 
      | an array to which midi controller-change
      | messages will be added. This will be
      | the minimum number of controller changes
      | to recreate the state at the required
      | time.
      |
      */
    pub fn create_controller_updates_for_time(&mut self, 
        channel_number: i32,
        time:           f64,
        dest:           &mut Vec<MidiMessage>)  {
        
        todo!();
        /*
            bool doneProg = false;
        bool donePitchWheel = false;
        bool doneControllers[128] = {};

        for (int i = list.size(); --i >= 0;)
        {
            auto& mm = list.getUnchecked(i)->message;

            if (mm.isForChannel (channelNumber) && mm.getTimeStamp() <= time)
            {
                if (mm.isProgramChange() && ! doneProg)
                {
                    doneProg = true;
                    dest.add (MidiMessage (mm, 0.0));
                }
                else if (mm.isPitchWheel() && ! donePitchWheel)
                {
                    donePitchWheel = true;
                    dest.add (MidiMessage (mm, 0.0));
                }
                else if (mm.isController())
                {
                    auto controllerNumber = mm.getControllerNumber();
                    jassert (isPositiveAndBelow (controllerNumber, 128));

                    if (! doneControllers[controllerNumber])
                    {
                        doneControllers[controllerNumber] = true;
                        dest.add (MidiMessage (mm, 0.0));
                    }
                }
            }
        }
        */
    }
}
