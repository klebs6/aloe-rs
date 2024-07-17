crate::ix!();

pub fn mark_chunk_meta_data_contains_zero_identifiers(values: &StringPairArray) -> bool {
    
    todo!();
        /*
            // (zero cue identifiers are valid for WAV but not for AIFF)
                const String cueString ("Cue");
                const String noteString ("CueNote");
                const String identifierString ("Identifier");

                for (auto& key : values.getAllKeys())
                {
                    if (key.startsWith (noteString))
                        continue; // zero identifier IS valid in a COMT chunk

                    if (key.startsWith (cueString) && key.contains (identifierString))
                        if (values.getValue (key, "-1").getIntValue() == 0)
                            return true;
                }

                return false;
        */
}

pub fn mark_chunk_create(
    block:  &mut MemoryBlock,
    values: &StringPairArray

) {
    
    todo!();
        /*
            auto numCues = values.getValue ("NumCuePoints", "0").getIntValue();

                if (numCues > 0)
                {
                    MemoryOutputStream out (block, false);
                    out.writeShortBigEndian ((short) numCues);

                    auto numCueLabels = values.getValue ("NumCueLabels", "0").getIntValue();
                    auto idOffset = metaDataContainsZeroIdentifiers (values) ? 1 : 0; // can't have zero IDs in AIFF

                   #if ALOE_DEBUG
                    Vec<int> identifiers;
                   #endif

                    for (int i = 0; i < numCues; ++i)
                    {
                        auto prefixCue = "Cue" + String (i);
                        auto identifier = idOffset + values.getValue (prefixCue + "Identifier", "1").getIntValue();

                       #if ALOE_DEBUG
                        jassert (! identifiers.contains (identifier));
                        identifiers.add (identifier);
                       #endif

                        auto offset = values.getValue (prefixCue + "Offset", "0").getIntValue();
                        auto label = "CueLabel" + String (i);

                        for (int labelIndex = 0; labelIndex < numCueLabels; ++labelIndex)
                        {
                            auto prefixLabel = "CueLabel" + String (labelIndex);
                            auto labelIdentifier = idOffset + values.getValue (prefixLabel + "Identifier", "1").getIntValue();

                            if (labelIdentifier == identifier)
                            {
                                label = values.getValue (prefixLabel + "Text", label);
                                break;
                            }
                        }

                        out.writeShortBigEndian ((short) identifier);
                        out.writeIntBigEndian (offset);

                        auto labelLength = jmin ((size_t) 254, label.getNumBytesAsUTF8()); // seems to need null terminator even though it's a pstring
                        out.writeByte (static_cast<char> (labelLength + 1));
                        out.write (label.toUTF8(), labelLength);
                        out.writeByte (0);

                        if ((out.getDataSize() & 1) != 0)
                            out.writeByte (0);
                    }
                }
        */
}
