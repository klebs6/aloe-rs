crate::ix!();

pub struct TracktionChunk {

}

impl TracktionChunk {

    pub fn create_from(values: &StringPairArray) -> MemoryBlock {
        
        todo!();
        /*
            MemoryOutputStream out;
                auto s = values[WavAudioFormat::tracktionLoopInfo];

                if (s.isNotEmpty())
                {
                    out.writeString (s);

                    if ((out.getDataSize() & 1) != 0)
                        out.writeByte (0);
                }

                return out.getMemoryBlock();
        */
    }
}

