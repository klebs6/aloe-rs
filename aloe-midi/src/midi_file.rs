crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiFile.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiFile.cpp]

/**
  | Reads/writes standard midi format files.
  |
  | To read a midi file, create a MidiFile object
  | and call its readFrom() method. You can then
  | get the individual midi tracks from it using
  | the getTrack() method.
  |
  | To write a file, create a MidiFile object, add
  | some MidiMessageSequence objects to it using
  | the addTrack() method, and then call its
  | writeTo() method to stream it out.
  |
  | @see MidiMessageSequence
  |
  | @tags{Audio}
  */
#[leak_detector]
pub struct MidiFile {
    tracks:      Vec<Box<MidiMessageSequence>>,
    time_format: i16,
}

impl Default for MidiFile {

    /// Creates an empty MidiFile object. 
    fn default() -> Self {

        todo!();
        /*
            : timeFormat ((short) (unsigned short) 0xe728
        */
    }
}

impl MidiFile {

    /**
      | Creates a copy of another MidiFile.
      |
      */
    pub fn new_from_other_ref(other: &MidiFile) -> Self {
    
        todo!();
        /*
        : time_format(other.timeFormat),

            tracks.addCopiesOf (other.tracks);
        */
    }
    
    /**
      | Copies from another MidiFile object
      |
      */
    pub fn assign_from_other_ref(&mut self, other: &MidiFile) -> &mut MidiFile {
        
        todo!();
        /*
            tracks.clear();
        tracks.addCopiesOf (other.tracks);
        timeFormat = other.timeFormat;
        return *this;
        */
    }
    
    pub fn new_from_other(other: MidiFile) -> Self {
    
        todo!();
        /*
        : tracks(std::move (other.tracks)),
        : time_format(other.timeFormat),

        
        */
    }
    
    /**
      | Copies from another MidiFile object
      |
      */
    pub fn assign_from_other(&mut self, other: MidiFile) -> &mut MidiFile {
        
        todo!();
        /*
            tracks = std::move (other.tracks);
        timeFormat = other.timeFormat;
        return *this;
        */
    }
    
    /** 
      | Removes all midi tracks from the file.
      |
      | @see getNumTracks
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            tracks.clear();
        */
    }
    
    /**
      | Returns the number of tracks in the file.
      | @see getTrack, addTrack
      |
      */
    pub fn get_num_tracks(&self) -> i32 {
        
        todo!();
        /*
            return tracks.size();
        */
    }
    
    /** 
      | Returns a pointer to one of the tracks in the
      | file.
      |
      | @returns a pointer to the track, or
      | nullptr if the index is out-of-range
      |
      | @see getNumTracks, addTrack
      */
    pub fn get_track(&self, index: i32) -> *const MidiMessageSequence {
        
        todo!();
        /*
            return tracks[index];
        */
    }
    
    /** 
      | Adds a midi track to the file.
      |
      | This will make its own internal copy of the
      | sequence that is passed-in.
      |
      | @see getNumTracks, getTrack
      */
    pub fn add_track(&mut self, track_sequence: &MidiMessageSequence)  {
        
        todo!();
        /*
            tracks.add (new MidiMessageSequence (trackSequence));
        */
    }
    
    /** 
      | Returns the raw time format code that will be
      | written to a stream.
      |
      | After reading a midi file, this method
      | will return the time-format that was read
      | from the file's header. It can be changed
      | using the setTicksPerQuarterNote() or
      | setSmpteTimeFormat() methods.
      |
      | If the value returned is positive, it
      | indicates the number of midi ticks per
      | quarter-note - see
      | setTicksPerQuarterNote().
      |
      | It it's negative, the upper byte indicates
      | the frames-per-second (but negative), and
      | the lower byte is the number of ticks per
      | frame - see setSmpteTimeFormat().
      */
    pub fn get_time_format(&self) -> i16 {
        
        todo!();
        /*
            return timeFormat;
        */
    }
    
    /**
      | Sets the time format to use when this
      | file is written to a stream.
      | 
      | If this is called, the file will be written
      | as bars/beats using the
      | 
      | specified resolution, rather than
      | SMPTE absolute times, as would be
      | 
      | used if setSmpteTimeFormat() had been
      | called instead.
      | 
      | -----------
      | @param ticksPerQuarterNote
      | 
      | e.g. 96, 960
      | 
      | @see setSmpteTimeFormat
      |
      */
    pub fn set_ticks_per_quarter_note(&mut self, ticks: i32)  {
        
        todo!();
        /*
            timeFormat = (short) ticks;
        */
    }
    
    /**
      | Sets the time format to use when this
      | file is written to a stream.
      | 
      | -----------
      | @note
      | 
      | If this is called, the file will be written
      | using absolute times, rather
      | 
      | than bars/beats as would be the case
      | if setTicksPerBeat() had been called
      | 
      | instead.
      | 
      | -----------
      | @param framesPerSecond
      | 
      | must be 24, 25, 29 or 30
      | ----------
      | @param subframeResolution
      | 
      | the sub-second resolution, e.g. 4 (midi
      | time code), 8, 10, 80 (SMPTE bit resolution),
      | or 100. For millisecond timing, setSmpteTimeFormat
      | (25, 40)
      | 
      | @see setTicksPerBeat
      |
      */
    pub fn set_smpte_time_format(&mut self, 
        frames_per_second:   i32,
        subframe_resolution: i32)  {
        
        todo!();
        /*
            timeFormat = (short) (((-framesPerSecond) << 8) | subframeResolution);
        */
    }
    
    /**
      | Makes a list of all the tempo-change
      | meta-events from all tracks in the midi
      | file. Useful for finding the positions
      | of all the tempo changes in a file.
      | 
      | -----------
      | @param tempoChangeEvents
      | 
      | a list to which all the events will be
      | added
      |
      */
    pub fn find_all_tempo_events(&self, results: &mut MidiMessageSequence)  {
        
        todo!();
        /*
            MidiFileHelpers::findAllMatchingEvents (tracks, results, &MidiMessage::isTempoMetaEvent);
        */
    }
    
    /**
      | Makes a list of all the time-signature
      | meta-events from all tracks in the midi
      | file. Useful for finding the positions
      | of all the tempo changes in a file.
      | 
      | -----------
      | @param timeSigEvents
      | 
      | a list to which all the events will be
      | added
      |
      */
    pub fn find_all_time_sig_events(&self, results: &mut MidiMessageSequence)  {
        
        todo!();
        /*
            MidiFileHelpers::findAllMatchingEvents (tracks, results, &MidiMessage::isTimeSignatureMetaEvent);
        */
    }
    
    /**
      | Makes a list of all the time-signature
      | meta-events from all tracks in the midi
      | file.
      | 
      | -----------
      | @param keySigEvents
      | 
      | a list to which all the events will be
      | added
      |
      */
    pub fn find_all_key_sig_events(&self, results: &mut MidiMessageSequence)  {
        
        todo!();
        /*
            MidiFileHelpers::findAllMatchingEvents (tracks, results, &MidiMessage::isKeySignatureMetaEvent);
        */
    }
    
    /**
      | Returns the latest timestamp in any
      | of the tracks. (Useful for finding the
      | length of the file).
      */
    pub fn get_last_timestamp(&self) -> f64 {
        
        todo!();
        /*
            double t = 0.0;

        for (auto* ms : tracks)
            t = jmax (t, ms->getEndTime());

        return t;
        */
    }
    
    /**
      | Reads a midi file format stream.
      | 
      | After calling this, you can get the tracks
      | that were read from the file by using
      | the
      | 
      | getNumTracks() and getTrack() methods.
      | 
      | The timestamps of the midi events in
      | the tracks will represent their positions
      | in terms of midi ticks. To convert them
      | to seconds, use the 
      | convertTimestampTicksToSeconds() method.
      | 
      | -----------
      | @param sourceStream
      | 
      | the source stream
      | ----------
      | @param createMatchingNoteOffs
      | 
      | if true, any missing note-offs for previous
      | note-ons will be automatically added
      | at the end of the file by calling 
      | MidiMessageSequence::updateMatchedPairs
      | on each track.
      | ----------
      | @param midiFileType
      | 
      | if not nullptr, the integer at this address
      | will be set to 0, 1, or 2 depending on the
      | type of the midi file
      | 
      | -----------
      | @return
      | 
      | true if the stream was read successfully
      |
      */
    pub fn read_from<R: Read>(
        &mut self, 
        source_stream:             &mut R,
        create_matching_note_offs: Option<bool>,
        file_type:                 *mut i32

    ) -> bool {

        let create_matching_note_offs: bool = create_matching_note_offs.unwrap_or(true);
        
        todo!();
        /*
            clear();
        MemoryBlock data;

        const int maxSensibleMidiFileSize = 200 * 1024 * 1024;

        // (put a sanity-check on the file size, as midi files are generally small)
        if (! sourceStream.readIntoMemoryBlock (data, maxSensibleMidiFileSize))
            return false;

        auto size = data.getSize();
        auto d = static_cast<const uint8*> (data.getData());

        const auto optHeader = MidiFileHelpers::parseMidiHeader (d, size);

        if (! optHeader.valid)
            return false;

        const auto header = optHeader.value;
        timeFormat = header.timeFormat;

        d += header.bytesRead;
        size -= (size_t) header.bytesRead;

        for (int track = 0; track < header.numberOfTracks; ++track)
        {
            const auto optChunkType = MidiFileHelpers::tryRead<uint32> (d, size);

            if (! optChunkType.valid)
                return false;

            const auto optChunkSize = MidiFileHelpers::tryRead<uint32> (d, size);

            if (! optChunkSize.valid)
                return false;

            const auto chunkSize = optChunkSize.value;

            if (size < chunkSize)
                return false;

            if (optChunkType.value == ByteOrder::bigEndianInt ("MTrk"))
                readNextTrack (d, (int) chunkSize, createMatchingNoteOffs);

            size -= chunkSize;
            d += chunkSize;
        }

        const auto successful = (size == 0);

        if (successful && fileType != nullptr)
            *fileType = header.fileType;

        return successful;
        */
    }
    
    pub fn read_next_track(&mut self, 
        data:                      *const u8,
        size:                      i32,
        create_matching_note_offs: bool)  {
        
        todo!();
        /*
            auto sequence = MidiFileHelpers::readTrack (data, size);

        // sort so that we put all the note-offs before note-ons that have the same time
        std::stable_sort (sequence.list.begin(), sequence.list.end(),
                          [] (const MidiMessageSequence::MidiEventHolder* a,
                              const MidiMessageSequence::MidiEventHolder* b)
        {
            auto t1 = a->message.getTimeStamp();
            auto t2 = b->message.getTimeStamp();

            if (t1 < t2)  return true;
            if (t2 < t1)  return false;

            return a->message.isNoteOff() && b->message.isNoteOn();
        });

        if (createMatchingNoteOffs)
            sequence.updateMatchedPairs();

        addTrack (sequence);
        */
    }
    
    /**
      | Converts the timestamp of all the midi
      | events from midi ticks to seconds.
      | 
      | This will use the midi time format and
      | tempo/time signature info in the 
      | tracks to convert all the timestamps
      | to absolute values in seconds.
      |
      */
    pub fn convert_timestamp_ticks_to_seconds(&mut self)  {
        
        todo!();
        /*
            MidiMessageSequence tempoEvents;
        findAllTempoEvents (tempoEvents);
        findAllTimeSigEvents (tempoEvents);

        if (timeFormat != 0)
        {
            for (auto* ms : tracks)
            {
                for (int j = ms->getNumEvents(); --j >= 0;)
                {
                    auto& m = ms->getEventPointer(j)->message;
                    m.setTimeStamp (MidiFileHelpers::convertTicksToSeconds (m.getTimeStamp(), tempoEvents, timeFormat));
                }
            }
        }
        */
    }
    
    /**
      | Writes the midi tracks as a standard
      | midi file. The midiFileType value is
      | written as the file's format type, which
      | can be 0, 1 or 2 - see the midi file spec
      | for more info about that.
      | 
      | -----------
      | @param destStream
      | 
      | the destination stream
      | ----------
      | @param midiFileType
      | 
      | the type of midi file
      | 
      | -----------
      | @return
      | 
      | true if the operation succeeded.
      |
      */
    pub fn write_to<W: Write>(
        &self, 
        out:            &mut W,
        midi_file_type: Option<i32>

    ) -> bool {

        let midi_file_type = midi_file_type.unwrap_or(1);
        
        todo!();
        /*
            jassert (midiFileType >= 0 && midiFileType <= 2);

        if (! out.writeIntBigEndian ((int) ByteOrder::bigEndianInt ("MThd"))) return false;
        if (! out.writeIntBigEndian (6))                                      return false;
        if (! out.writeShortBigEndian ((short) midiFileType))                 return false;
        if (! out.writeShortBigEndian ((short) tracks.size()))                return false;
        if (! out.writeShortBigEndian (timeFormat))                           return false;

        for (auto* ms : tracks)
            if (! writeTrack (out, *ms))
                return false;

        out.flush();
        return true;
        */
    }
    
    pub fn write_track<W: Write>(
        &self, 
        main_out: &mut W,
        ms:       &MidiMessageSequence

    ) -> bool {
        
        todo!();
        /*
            MemoryOutputStream out;

        int lastTick = 0;
        uint8 lastStatusByte = 0;
        bool endOfTrackEventWritten = false;

        for (int i = 0; i < ms.getNumEvents(); ++i)
        {
            auto& mm = ms.getEventPointer(i)->message;

            if (mm.isEndOfTrackMetaEvent())
                endOfTrackEventWritten = true;

            auto tick = roundToInt (mm.getTimeStamp());
            auto delta = jmax (0, tick - lastTick);
            MidiFileHelpers::writeVariableLengthInt (out, (uint32) delta);
            lastTick = tick;

            auto* data = mm.getRawData();
            auto dataSize = mm.getRawDataSize();
            auto statusByte = data[0];

            if (statusByte == lastStatusByte
                 && (statusByte & 0xf0) != 0xf0
                 && dataSize > 1
                 && i > 0)
            {
                ++data;
                --dataSize;
            }
            else if (statusByte == 0xf0)  // Write sysex message with length bytes.
            {
                out.writeByte ((char) statusByte);

                ++data;
                --dataSize;

                MidiFileHelpers::writeVariableLengthInt (out, (uint32) dataSize);
            }

            out.write (data, (size_t) dataSize);
            lastStatusByte = statusByte;
        }

        if (! endOfTrackEventWritten)
        {
            out.writeByte (0); // (tick delta)
            auto m = MidiMessage::endOfTrack();
            out.write (m.getRawData(), (size_t) m.getRawDataSize());
        }

        if (! mainOut.writeIntBigEndian ((int) ByteOrder::bigEndianInt ("MTrk"))) return false;
        if (! mainOut.writeIntBigEndian ((int) out.getDataSize()))                return false;

        mainOut << out;

        return true;
        */
    }
}
