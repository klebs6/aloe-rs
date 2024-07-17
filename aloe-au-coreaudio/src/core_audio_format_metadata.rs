crate::ix!();

pub fn core_audio_format_metadata_chunk_name(name: *const u8) -> u32 {
    
    todo!();
        /*
            return ByteOrder::bigEndianInt (name);
        */
}

pub fn core_audio_format_metadata_parse_user_defined_chunk<R: Read>(
    input: &mut R,
    size:  i64
) -> StringPairArray {
    
    todo!();
        /*
            StringPairArray infoStrings;
            auto originalPosition = input.getPosition();

            uint8 uuid[16];
            input.read (uuid, sizeof (uuid));

            if (memcmp (uuid, "\x29\x81\x92\x73\xB5\xBF\x4A\xEF\xB7\x8D\x62\xD1\xEF\x90\xBB\x2C", 16) == 0)
            {
                auto numEntries = (uint32) input.readIntBigEndian();

                for (uint32 i = 0; i < numEntries && input.getPosition() < originalPosition + size; ++i)
                {
                    String keyName = input.readString();
                    infoStrings.set (keyName, input.readString());
                }
            }

            input.setPosition (originalPosition + size);
            return infoStrings;
        */
}

///------------------------------
pub fn core_audio_format_metadata_parse_midi_chunk<R: Read>(
    input: &mut R,
    size:  i64

) -> StringPairArray {
    
    todo!();
        /*
            auto originalPosition = input.getPosition();

            MemoryBlock midiBlock;
            input.readIntoMemoryBlock (midiBlock, (ssize_t) size);
            MemoryInputStream midiInputStream (midiBlock, false);

            StringPairArray midiMetadata;
            MidiFile midiFile;

            if (midiFile.readFrom (midiInputStream))
            {
                midiMetadata.set (CoreAudioFormat::midiDataBase64, midiBlock.toBase64Encoding());

                findTempoEvents (midiFile, midiMetadata);
                findTimeSigEvents (midiFile, midiMetadata);
                findKeySigEvents (midiFile, midiMetadata);
            }

            input.setPosition (originalPosition + size);
            return midiMetadata;
        */
}

///------------------------------
pub fn core_audio_format_metadata_find_tempo_events(
    midi_file:     &mut MidiFile,
    midi_metadata: &mut StringPairArray
) {
    
    todo!();
        /*
            MidiMessageSequence tempoEvents;
            midiFile.findAllTempoEvents (tempoEvents);

            auto numTempoEvents = tempoEvents.getNumEvents();
            MemoryOutputStream tempoSequence;

            for (int i = 0; i < numTempoEvents; ++i)
            {
                auto tempo = getTempoFromTempoMetaEvent (tempoEvents.getEventPointer (i));

                if (tempo > 0.0)
                {
                    if (i == 0)
                        midiMetadata.set (CoreAudioFormat::tempo, String (tempo));

                    if (numTempoEvents > 1)
                        tempoSequence << String (tempo) << ',' << tempoEvents.getEventTime (i) << ';';
                }
            }

            if (tempoSequence.getDataSize() > 0)
                midiMetadata.set ("tempo sequence", tempoSequence.toUTF8());
        */
}

///------------------------------
pub fn core_audio_format_metadata_get_tempo_from_tempo_meta_event(holder: *mut MidiMessageSequenceMidiEventHolder) -> f64 {
    
    todo!();
        /*
            if (holder != nullptr)
            {
                auto& midiMessage = holder->message;

                if (midiMessage.isTempoMetaEvent())
                {
                    auto tempoSecondsPerQuarterNote = midiMessage.getTempoSecondsPerQuarterNote();

                    if (tempoSecondsPerQuarterNote > 0.0)
                        return 60.0 / tempoSecondsPerQuarterNote;
                }
            }

            return 0.0;
        */
}

///------------------------------
pub fn core_audio_format_metadata_find_time_sig_events(
        midi_file:     &mut MidiFile,
        midi_metadata: &mut StringPairArray)  {
    
    todo!();
        /*
            MidiMessageSequence timeSigEvents;
            midiFile.findAllTimeSigEvents (timeSigEvents);
            auto numTimeSigEvents = timeSigEvents.getNumEvents();

            MemoryOutputStream timeSigSequence;

            for (int i = 0; i < numTimeSigEvents; ++i)
            {
                int numerator, denominator;
                timeSigEvents.getEventPointer(i)->message.getTimeSignatureInfo (numerator, denominator);

                String timeSigString;
                timeSigString << numerator << '/' << denominator;

                if (i == 0)
                    midiMetadata.set (CoreAudioFormat::timeSig, timeSigString);

                if (numTimeSigEvents > 1)
                    timeSigSequence << timeSigString << ',' << timeSigEvents.getEventTime (i) << ';';
            }

            if (timeSigSequence.getDataSize() > 0)
                midiMetadata.set ("time signature sequence", timeSigSequence.toUTF8());
        */
}

///------------------------------
pub fn core_audio_format_metadata_find_key_sig_events(
        midi_file:     &mut MidiFile,
        midi_metadata: &mut StringPairArray)  {
    
    todo!();
        /*
            MidiMessageSequence keySigEvents;
            midiFile.findAllKeySigEvents (keySigEvents);
            auto numKeySigEvents = keySigEvents.getNumEvents();

            MemoryOutputStream keySigSequence;

            for (int i = 0; i < numKeySigEvents; ++i)
            {
                auto& message (keySigEvents.getEventPointer (i)->message);
                auto key = jlimit (0, 14, message.getKeySignatureNumberOfSharpsOrFlats() + 7);
                bool isMajor = message.isKeySignatureMajorKey();

                static const char* majorKeys[] = { "Cb", "Gb", "Db", "Ab", "Eb", "Bb", "F", "C", "G", "D", "A", "E", "B", "F#", "C#" };
                static const char* minorKeys[] = { "Ab", "Eb", "Bb", "F", "C", "G", "D", "A", "E", "B", "F#", "C#", "G#", "D#", "A#" };

                String keySigString (isMajor ? majorKeys[key]
                                             : minorKeys[key]);

                if (! isMajor)
                    keySigString << 'm';

                if (i == 0)
                    midiMetadata.set (CoreAudioFormat::keySig, keySigString);

                if (numKeySigEvents > 1)
                    keySigSequence << keySigString << ',' << keySigEvents.getEventTime (i) << ';';
            }

            if (keySigSequence.getDataSize() > 0)
                midiMetadata.set ("key signature sequence", keySigSequence.toUTF8());
        */
}

///------------------------------
pub fn core_audio_format_metadata_parse_information_chunk<R: Read>(input: &mut R) -> StringPairArray {
    
    todo!();
        /*
            StringPairArray infoStrings;
            auto numEntries = (uint32) input.readIntBigEndian();

            for (uint32 i = 0; i < numEntries; ++i)
                infoStrings.set (input.readString(), input.readString());

            return infoStrings;
        */
}

///------------------------------
pub fn core_audio_format_metadata_read<R: Read>(
        input:           &mut R,
        metadata_values: &mut StringPairArray) -> bool {
    
    todo!();
        /*
            auto originalPos = input.getPosition();

            const CoreAudioFormatMetadataFileHeader cafFileHeader (input);
            const bool isCafFile = cafFileHeader.fileType == chunkName ("caff");

            if (isCafFile)
            {
                while (! input.isExhausted())
                {
                    const CoreAudioFormatMetadataChunkHeader chunkHeader (input);

                    if (chunkHeader.chunkType == chunkName ("desc"))
                    {
                        CoreAudioFormatMetadataAudioDescriptionChunk audioDescriptionChunk (input);
                    }
                    else if (chunkHeader.chunkType == chunkName ("uuid"))
                    {
                        metadataValues.addArray (parseUserDefinedChunk (input, chunkHeader.chunkSize));
                    }
                    else if (chunkHeader.chunkType == chunkName ("data"))
                    {
                        // -1 signifies an unknown data size so the data has to be at the
                        // end of the file so we must have finished the header

                        if (chunkHeader.chunkSize == -1)
                            break;

                        input.setPosition (input.getPosition() + chunkHeader.chunkSize);
                    }
                    else if (chunkHeader.chunkType == chunkName ("midi"))
                    {
                        metadataValues.addArray (parseMidiChunk (input, chunkHeader.chunkSize));
                    }
                    else if (chunkHeader.chunkType == chunkName ("info"))
                    {
                        metadataValues.addArray (parseInformationChunk (input));
                    }
                    else
                    {
                        // we aren't decoding this chunk yet so just skip over it
                        input.setPosition (input.getPosition() + chunkHeader.chunkSize);
                    }
                }
            }

            input.setPosition (originalPos);

            return isCafFile;
        */
}
