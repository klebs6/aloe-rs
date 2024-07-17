crate::ix!();

pub fn list_chunk_get_value(
    values: &StringPairArray,
    name:   &String) -> i32 {
    
    todo!();
        /*
            return values.getValue (name, "0").getIntValue();
        */
}

pub fn list_chunk_get_value_with_prefix(
    values: &StringPairArray,
    prefix: &String,
    name:   *const u8) -> i32 {

    todo!();
        /*
            return getValue (values, prefix + name);
        */
}

pub fn list_chunk_append_label_or_note_chunk(
    values:     &StringPairArray,
    prefix:     &String,
    chunk_type: i32,
    out:        &mut MemoryOutputStream
)  {

    todo!();
        /*
            auto label = values.getValue (prefix + "Text", prefix);
                auto labelLength = (int) label.getNumBytesAsUTF8() + 1;
                auto chunkLength = 4 + labelLength + (labelLength & 1);

                out.writeInt (chunkType);
                out.writeInt (chunkLength);
                out.writeInt (getValue (values, prefix, "Identifier"));
                out.write (label.toUTF8(), (size_t) labelLength);

                if ((out.getDataSize() & 1) != 0)
                    out.writeByte (0);
        */
}

pub fn list_chunk_append_extra_chunk(
    values: &StringPairArray,
    prefix: &String,
    out:    &mut MemoryOutputStream

) {
    
    todo!();
        /*
            auto text = values.getValue (prefix + "Text", prefix);

                auto textLength = (int) text.getNumBytesAsUTF8() + 1; // include null terminator
                auto chunkLength = textLength + 20 + (textLength & 1);

                out.writeInt (chunkName ("ltxt"));
                out.writeInt (chunkLength);
                out.writeInt (getValue (values, prefix, "Identifier"));
                out.writeInt (getValue (values, prefix, "SampleLength"));
                out.writeInt (getValue (values, prefix, "Purpose"));
                out.writeShort ((short) getValue (values, prefix, "Country"));
                out.writeShort ((short) getValue (values, prefix, "Language"));
                out.writeShort ((short) getValue (values, prefix, "Dialect"));
                out.writeShort ((short) getValue (values, prefix, "CodePage"));
                out.write (text.toUTF8(), (size_t) textLength);

                if ((out.getDataSize() & 1) != 0)
                    out.writeByte (0);
        */
}

pub fn list_chunk_create_from(values: &StringPairArray) -> MemoryBlock {
    
    todo!();
        /*
            auto numCueLabels  = getValue (values, "NumCueLabels");
                auto numCueNotes   = getValue (values, "NumCueNotes");
                auto numCueRegions = getValue (values, "NumCueRegions");

                MemoryOutputStream out;

                if (numCueLabels + numCueNotes + numCueRegions > 0)
                {
                    out.writeInt (chunkName ("adtl"));

                    for (int i = 0; i < numCueLabels; ++i)
                        appendLabelOrNoteChunk (values, "CueLabel" + String (i), chunkName ("labl"), out);

                    for (int i = 0; i < numCueNotes; ++i)
                        appendLabelOrNoteChunk (values, "CueNote" + String (i), chunkName ("note"), out);

                    for (int i = 0; i < numCueRegions; ++i)
                        appendExtraChunk (values, "CueRegion" + String (i), out);
                }

                return out.getMemoryBlock();
        */
}
