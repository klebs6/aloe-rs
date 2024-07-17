crate::ix!();

//#[bitfield]
pub struct InstChunk {
    base_note:     i8,
    detune:        i8,
    gain:          i8,
    low_note:      i8,
    high_note:     i8,
    low_velocity:  i8,
    high_velocity: i8,
}

impl InstChunk {

    pub fn set_value(
        values: &mut StringPairArray,
        name:   *const u8,
        val:    i32)  {
        
        todo!();
        /*
            values.set (name, String (val));
        */
    }
    
    pub fn copy_to(&self, values: &mut StringPairArray)  {
        
        todo!();
        /*
            setValue (values, "MidiUnityNote",  baseNote);
                setValue (values, "Detune",         detune);
                setValue (values, "Gain",           gain);
                setValue (values, "LowNote",        lowNote);
                setValue (values, "HighNote",       highNote);
                setValue (values, "LowVelocity",    lowVelocity);
                setValue (values, "HighVelocity",   highVelocity);
        */
    }
    
    pub fn get_value_with_name_and_def(
        values: &StringPairArray,
        name:   *const u8,
        def:    *const u8) -> i8 {
        
        todo!();
        /*
            return (int8) values.getValue (name, def).getIntValue();
        */
    }
    
    pub fn create_from(values: &StringPairArray) -> MemoryBlock {
        
        todo!();
        /*
            MemoryBlock data;
                auto& keys = values.getAllKeys();

                if (keys.contains ("LowNote", true) && keys.contains ("HighNote", true))
                {
                    data.setSize (8, true);
                    auto* inst = static_cast<InstChunk*> (data.getData());

                    inst->baseNote      = getValue (values, "MidiUnityNote", "60");
                    inst->detune        = getValue (values, "Detune", "0");
                    inst->gain          = getValue (values, "Gain", "0");
                    inst->lowNote       = getValue (values, "LowNote", "0");
                    inst->highNote      = getValue (values, "HighNote", "127");
                    inst->lowVelocity   = getValue (values, "LowVelocity", "1");
                    inst->highVelocity  = getValue (values, "HighVelocity", "127");
                }

                return data;
        */
    }
}
