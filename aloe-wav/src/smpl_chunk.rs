crate::ix!();

//#[bitfield]
pub struct SmplChunkSampleLoop
{
    identifier: u32,

    /**
      | these are different in AIFF and WAV
      |
      */
    ty:         u32,

    start:      u32,
    end:        u32,
    fraction:   u32,
    play_count: u32,
}

//#[bitfield]
pub struct SMPLChunk {
    manufacturer:        u32,
    product:             u32,
    sample_period:       u32,
    midi_unity_note:     u32,
    midi_pitch_fraction: u32,
    smpte_format:        u32,
    smpte_offset:        u32,
    num_sample_loops:    u32,
    sampler_data:        u32,
    loops:               [SmplChunkSampleLoop; 1],
}

impl SMPLChunk {

    pub fn set_value<NameType>(
        values: &mut StringPairArray,
        name:   NameType,
        val:    u32)  {
    
        todo!();
        /*
            values.set (name, String (ByteOrder::swapIfBigEndian (val)));
        */
    }
    
    pub fn set_value_with_prefix(
        values: &mut StringPairArray,
        prefix: i32,
        name:   *const u8,
        val:    u32)  {
        
        todo!();
        /*
            setValue (values, "Loop" + String (prefix) + name, val);
        */
    }
    
    pub fn copy_to(&self, 
        values:     &mut StringPairArray,
        total_size: i32)  {
        
        todo!();
        /*
            setValue (values, "Manufacturer",      manufacturer);
                setValue (values, "Product",           product);
                setValue (values, "SamplePeriod",      samplePeriod);
                setValue (values, "MidiUnityNote",     midiUnityNote);
                setValue (values, "MidiPitchFraction", midiPitchFraction);
                setValue (values, "SmpteFormat",       smpteFormat);
                setValue (values, "SmpteOffset",       smpteOffset);
                setValue (values, "NumSampleLoops",    numSampleLoops);
                setValue (values, "SamplerData",       samplerData);

                for (int i = 0; i < (int) numSampleLoops; ++i)
                {
                    if ((uint8*) (loops + (i + 1)) > ((uint8*) this) + totalSize)
                        break;

                    setValue (values, i, "Identifier", loops[i].identifier);
                    setValue (values, i, "Type",       loops[i].type);
                    setValue (values, i, "Start",      loops[i].start);
                    setValue (values, i, "End",        loops[i].end);
                    setValue (values, i, "Fraction",   loops[i].fraction);
                    setValue (values, i, "PlayCount",  loops[i].playCount);
                }
        */
    }
    
    pub fn get_value_with_name_type_and_def<NameType>(
        values: &StringPairArray,
        name:   NameType,
        def:    *const u8) -> u32 {
    
        todo!();
        /*
            return ByteOrder::swapIfBigEndian ((uint32) values.getValue (name, def).getIntValue());
        */
    }
    
    pub fn get_value_with_prefix(
        values: &StringPairArray,
        prefix: i32,
        name:   *const u8,
        def:    *const u8) -> u32 {
        
        todo!();
        /*
            return getValue (values, "Loop" + String (prefix) + name, def);
        */
    }
    
    pub fn create_from(values: &StringPairArray) -> MemoryBlock {
        
        todo!();
        /*
            MemoryBlock data;
                auto numLoops = jmin (64, values.getValue ("NumSampleLoops", "0").getIntValue());

                data.setSize (roundUpSize (sizeof (SMPLChunk) + (size_t) (jmax (0, numLoops - 1)) * sizeof (SmplChunkSampleLoop)), true);

                auto s = static_cast<SMPLChunk*> (data.getData());

                s->manufacturer      = getValue (values, "Manufacturer", "0");
                s->product           = getValue (values, "Product", "0");
                s->samplePeriod      = getValue (values, "SamplePeriod", "0");
                s->midiUnityNote     = getValue (values, "MidiUnityNote", "60");
                s->midiPitchFraction = getValue (values, "MidiPitchFraction", "0");
                s->smpteFormat       = getValue (values, "SmpteFormat", "0");
                s->smpteOffset       = getValue (values, "SmpteOffset", "0");
                s->numSampleLoops    = ByteOrder::swapIfBigEndian ((uint32) numLoops);
                s->samplerData       = getValue (values, "SamplerData", "0");

                for (int i = 0; i < numLoops; ++i)
                {
                    auto& loop = s->loops[i];
                    loop.identifier = getValue (values, i, "Identifier", "0");
                    loop.type       = getValue (values, i, "Type", "0");
                    loop.start      = getValue (values, i, "Start", "0");
                    loop.end        = getValue (values, i, "End", "0");
                    loop.fraction   = getValue (values, i, "Fraction", "0");
                    loop.playCount  = getValue (values, i, "PlayCount", "0");
                }

                return data;
        */
    }
}
