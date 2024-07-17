crate::ix!();

//#[bitfield]
pub struct AcidChunk {
    flags:             u32,
    root_note:         u16,
    reserved1:         u16,
    reserved2:         f32,
    num_beats:         u32,
    meter_denominator: u16,
    meter_numerator:   u16,
    tempo:             f32,
}

impl AcidChunk {

    /**
      | Reads an acid RIFF chunk from a stream
      | positioned just after the size byte.
      |
      */
    pub fn new_with_reader<R: Read>(
        input:  &mut R,
        length: usize

    ) -> Self {
    
        todo!();
        /*


            zerostruct (*this);
                input.read (this, (int) jmin (sizeof (*this), length));
        */
    }
    
    pub fn new_from_values(values: &Vec<(String,String)>) -> Self {
    
        todo!();
        /*


            zerostruct (*this);

                flags = getFlagIfPresent (values, WavAudioFormat::acidOneShot,   0x01)
                      | getFlagIfPresent (values, WavAudioFormat::acidRootSet,   0x02)
                      | getFlagIfPresent (values, WavAudioFormat::acidStretch,   0x04)
                      | getFlagIfPresent (values, WavAudioFormat::acidDiskBased, 0x08)
                      | getFlagIfPresent (values, WavAudioFormat::acidizerFlag,  0x10);

                if (values[WavAudioFormat::acidRootSet].getIntValue() != 0)
                    rootNote = ByteOrder::swapIfBigEndian ((uint16) values[WavAudioFormat::acidRootNote].getIntValue());

                numBeats          = ByteOrder::swapIfBigEndian ((uint32) values[WavAudioFormat::acidBeats].getIntValue());
                meterDenominator  = ByteOrder::swapIfBigEndian ((uint16) values[WavAudioFormat::acidDenominator].getIntValue());
                meterNumerator    = ByteOrder::swapIfBigEndian ((uint16) values[WavAudioFormat::acidNumerator].getIntValue());

                if (values.containsKey (WavAudioFormat::acidTempo))
                    tempo = swapFloatByteOrder (values[WavAudioFormat::acidTempo].getFloatValue());
        */
    }
    
    pub fn create_from(values: &Vec<(String,String)>) -> MemoryBlock {
        
        todo!();
        /*
            return AcidChunk (values).toMemoryBlock();
        */
    }
    
    pub fn to_memory_block(&self) -> MemoryBlock {
        
        todo!();
        /*
            return (flags != 0 || rootNote != 0 || numBeats != 0 || meterDenominator != 0 || meterNumerator != 0)
                          ? MemoryBlock (this, sizeof (*this)) : MemoryBlock();
        */
    }
    
    pub fn add_to_metadata(&self, values: &mut Vec<(String,String)>)  {
        
        todo!();
        /*
            setBoolFlag (values, WavAudioFormat::acidOneShot,   0x01);
                setBoolFlag (values, WavAudioFormat::acidRootSet,   0x02);
                setBoolFlag (values, WavAudioFormat::acidStretch,   0x04);
                setBoolFlag (values, WavAudioFormat::acidDiskBased, 0x08);
                setBoolFlag (values, WavAudioFormat::acidizerFlag,  0x10);

                if (flags & 0x02) // root note set
                    values.set (WavAudioFormat::acidRootNote, String (ByteOrder::swapIfBigEndian (rootNote)));

                values.set (WavAudioFormat::acidBeats,       String (ByteOrder::swapIfBigEndian (numBeats)));
                values.set (WavAudioFormat::acidDenominator, String (ByteOrder::swapIfBigEndian (meterDenominator)));
                values.set (WavAudioFormat::acidNumerator,   String (ByteOrder::swapIfBigEndian (meterNumerator)));
                values.set (WavAudioFormat::acidTempo,       String (swapFloatByteOrder (tempo)));
        */
    }
    
    pub fn set_bool_flag(
        &self, 
        values: &mut Vec<(String,String)>,
        name:   *const u8,
        mask:   u32

    ) {
        
        todo!();
        /*
            values.set (name, (flags & ByteOrder::swapIfBigEndian (mask)) ? "1" : "0");
        */
    }
    
    pub fn get_flag_if_present(
        values: &Vec<(String,String)>,
        name:   *const u8,
        flag:   u32

    ) -> u32 {
        
        todo!();
        /*
            return values[name].getIntValue() != 0 ? ByteOrder::swapIfBigEndian (flag) : 0;
        */
    }
    
    pub fn swap_float_byte_order(x: f32) -> f32 {
        
        todo!();
        /*
            #ifdef ALOE_BIG_ENDIAN
                union { uint32 asInt; float asFloat; } n;
                n.asFloat = x;
                n.asInt = ByteOrder::swap (n.asInt);
                return n.asFloat;
               #else
                return x;
               #endif
        */
    }
}
