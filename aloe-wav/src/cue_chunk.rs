crate::ix!();

#[bitfield]
pub struct CueChunkCue
{
    identifier:  u32,
    order:       u32,
    chunkid:     u32,
    chunk_start: u32,
    block_start: u32,
    offset:      u32,
}

//#[bitfield]
pub struct CueChunk {
    num_cues: u32,
    cues:     [CueChunkCue; 1],
}

impl CueChunk {

    pub fn set_value(
        values: &mut Vec<(String,String)>,
        prefix: i32,
        name:   *const u8,
        val:    u32)  {
        
        todo!();
        /*
            values.set ("CueChunkCue" + String (prefix) + name, String (ByteOrder::swapIfBigEndian (val)));
        */
    }
    
    pub fn copy_to(&self, 
        values:     &mut Vec<(String,String)>,
        total_size: i32)  {
        
        todo!();
        /*
            values.set ("NumCuePoints", String (ByteOrder::swapIfBigEndian (numCues)));

                for (int i = 0; i < (int) numCues; ++i)
                {
                    if ((uint8*) (cues + (i + 1)) > ((uint8*) this) + totalSize)
                        break;

                    setValue (values, i, "Identifier",  cues[i].identifier);
                    setValue (values, i, "Order",       cues[i].order);
                    setValue (values, i, "ChunkID",     cues[i].chunkID);
                    setValue (values, i, "ChunkStart",  cues[i].chunkStart);
                    setValue (values, i, "BlockStart",  cues[i].blockStart);
                    setValue (values, i, "Offset",      cues[i].offset);
                }
        */
    }
    
    pub fn create_from(values: &Vec<(String,String)>) -> MemoryBlock {
        
        todo!();
        /*
            MemoryBlock data;
                const int numCues = values.getValue ("NumCuePoints", "0").getIntValue();

                if (numCues > 0)
                {
                    data.setSize (roundUpSize (sizeof (CueChunk) + (size_t) (numCues - 1) * sizeof (CueChunkCue)), true);

                    auto c = static_cast<CueChunk*> (data.getData());

                    c->numCues = ByteOrder::swapIfBigEndian ((uint32) numCues);

                    const String dataChunkID (chunkName ("data"));
                    int nextOrder = 0;

                   #if ALOE_DEBUG
                    Vec<uint32> identifiers;
                   #endif

                    for (int i = 0; i < numCues; ++i)
                    {
                        auto prefix = "CueChunkCue" + String (i);
                        auto identifier = (uint32) values.getValue (prefix + "Identifier", "0").getIntValue();

                       #if ALOE_DEBUG
                        jassert (! identifiers.contains (identifier));
                        identifiers.add (identifier);
                       #endif

                        auto order = values.getValue (prefix + "Order", String (nextOrder)).getIntValue();
                        nextOrder = jmax (nextOrder, order) + 1;

                        auto& cue = c->cues[i];
                        cue.identifier   = ByteOrder::swapIfBigEndian ((uint32) identifier);
                        cue.order        = ByteOrder::swapIfBigEndian ((uint32) order);
                        cue.chunkID      = ByteOrder::swapIfBigEndian ((uint32) values.getValue (prefix + "ChunkID", dataChunkID).getIntValue());
                        cue.chunkStart   = ByteOrder::swapIfBigEndian ((uint32) values.getValue (prefix + "ChunkStart", "0").getIntValue());
                        cue.blockStart   = ByteOrder::swapIfBigEndian ((uint32) values.getValue (prefix + "BlockStart", "0").getIntValue());
                        cue.offset       = ByteOrder::swapIfBigEndian ((uint32) values.getValue (prefix + "Offset", "0").getIntValue());
                    }
                }

                return data;
        */
    }
}
