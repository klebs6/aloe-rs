crate::ix!();

#[inline] pub fn aiff_file_chunk_name(name: *const u8) -> i32 {
    
    todo!();
    /*
        return (int) ByteOrder::littleEndianInt (name);
    */
}

#[bitfield]
#[derive(BitfieldSpecifier)]
pub struct InstChunkLoop {

    /**
      | these are different in AIFF and WAV
      |
      */
    ty:               B16,

    start_identifier: B16,
    end_identifier:   B16,
}

#[bitfield]
pub struct InstChunk
{
    base_note:     B8,
    detune:        B8,
    low_note:      B8,
    high_note:     B8,
    low_velocity:  B8,
    high_velocity: B8,
    gain:          B16,
    sustain_loop:  InstChunkLoop,
    release_loop:  InstChunkLoop,
}

impl InstChunk {

    pub fn copy_to(&self, values: &mut HashMap<String,String>)  {
        
        todo!();
        /*
            values.emplace ("MidiUnityNote",        String (baseNote));
                values.emplace ("Detune",               String (detune));

                values.emplace ("LowNote",              String (lowNote));
                values.emplace ("HighNote",             String (highNote));
                values.emplace ("LowVelocity",          String (lowVelocity));
                values.emplace ("HighVelocity",         String (highVelocity));

                values.emplace ("Gain",                 String ((int16) ByteOrder::swapIfLittleEndian ((uint16) gain)));

                values.emplace ("NumSampleLoops",       String (2));        // always 2 with AIFF, WAV can have more
                values.emplace ("Loop0Type",            String (ByteOrder::swapIfLittleEndian (sustainLoop.type)));
                values.emplace ("Loop0StartIdentifier", String (ByteOrder::swapIfLittleEndian (sustainLoop.startIdentifier)));
                values.emplace ("Loop0EndIdentifier",   String (ByteOrder::swapIfLittleEndian (sustainLoop.endIdentifier)));
                values.emplace ("Loop1Type",            String (ByteOrder::swapIfLittleEndian (releaseLoop.type)));
                values.emplace ("Loop1StartIdentifier", String (ByteOrder::swapIfLittleEndian (releaseLoop.startIdentifier)));
                values.emplace ("Loop1EndIdentifier",   String (ByteOrder::swapIfLittleEndian (releaseLoop.endIdentifier)));
        */
    }
    
    pub fn get_value16(
        values: &StringPairArray,
        name:   *const u8,
        def:    *const u8) -> u16 {
        
        todo!();
        /*
            return ByteOrder::swapIfLittleEndian ((uint16) values.getValue (name, def).getIntValue());
        */
    }
    
    pub fn get_value8(
        values: &StringPairArray,
        name:   *const u8,
        def:    *const u8) -> i8 {
        
        todo!();
        /*
            return (int8) values.getValue (name, def).getIntValue();
        */
    }
    
    pub fn create(
        block:  &mut MemoryBlock,
        values: &StringPairArray)  {
        
        todo!();
        /*
            if (values.getAllKeys().contains ("MidiUnityNote", true))
                {
                    block.setSize ((sizeof (InstChunk) + 3) & ~(size_t) 3, true);
                    auto& inst = *static_cast<InstChunk*> (block.getData());

                    inst.baseNote      = getValue8 (values, "MidiUnityNote", "60");
                    inst.detune        = getValue8 (values, "Detune", "0");
                    inst.lowNote       = getValue8 (values, "LowNote", "0");
                    inst.highNote      = getValue8 (values, "HighNote", "127");
                    inst.lowVelocity   = getValue8 (values, "LowVelocity", "1");
                    inst.highVelocity  = getValue8 (values, "HighVelocity", "127");
                    inst.gain          = (int16) getValue16 (values, "Gain", "0");

                    inst.sustainLoop.type              = getValue16 (values, "Loop0Type", "0");
                    inst.sustainLoop.startIdentifier   = getValue16 (values, "Loop0StartIdentifier", "0");
                    inst.sustainLoop.endIdentifier     = getValue16 (values, "Loop0EndIdentifier", "0");
                    inst.releaseLoop.type              = getValue16 (values, "Loop1Type", "0");
                    inst.releaseLoop.startIdentifier   = getValue16 (values, "Loop1StartIdentifier", "0");
                    inst.releaseLoop.endIdentifier     = getValue16 (values, "Loop1EndIdentifier", "0");
                }
        */
    }
}
