crate::ix!();

pub fn comt_chunk_create(
    block:  &mut MemoryBlock,
    values: &StringPairArray

) {
    
    todo!();
        /*
            auto numNotes = values.getValue ("NumCueNotes", "0").getIntValue();

                if (numNotes > 0)
                {
                    MemoryOutputStream out (block, false);
                    out.writeShortBigEndian ((short) numNotes);

                    for (int i = 0; i < numNotes; ++i)
                    {
                        auto prefix = "CueNote" + String (i);

                        out.writeIntBigEndian (values.getValue (prefix + "TimeStamp", "0").getIntValue());
                        out.writeShortBigEndian ((short) values.getValue (prefix + "Identifier", "0").getIntValue());

                        auto comment = values.getValue (prefix + "Text", String());
                        auto commentLength = jmin (comment.getNumBytesAsUTF8(), (size_t) 65534);

                        out.writeShortBigEndian (static_cast<short> (commentLength + 1));
                        out.write (comment.toUTF8(), commentLength);
                        out.writeByte (0);

                        if ((out.getDataSize() & 1) != 0)
                            out.writeByte (0);
                    }
                }
        */
}
