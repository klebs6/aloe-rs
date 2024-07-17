crate::ix!();

pub enum BascChunkKey
{
    minor   = 1,
    major   = 2,
    neither = 3,
    both    = 4
}

//#[bitfield]
pub struct BASCChunk
{
    flags:        u32,
    num_beats:    u32,
    root_note:    u16,
    key:          u16,
    time_sig_num: u16,
    time_sig_den: u16,
    one_shot:     u16,
    unknown:      [u8; 66],
}

impl BASCChunk {

    pub fn new(input: &mut dyn Read) -> Self {
    
        todo!();
        /*


            zerostruct (*this);

                flags       = (uint32) input.readIntBigEndian();
                numBeats    = (uint32) input.readIntBigEndian();
                rootNote    = (uint16) input.readShortBigEndian();
                key         = (uint16) input.readShortBigEndian();
                timeSigNum  = (uint16) input.readShortBigEndian();
                timeSigDen  = (uint16) input.readShortBigEndian();
                oneShot     = (uint16) input.readShortBigEndian();
                input.read (unknown, sizeof (unknown));
        */
    }
    
    pub fn add_to_metadata(&self, metadata: &mut HashMap<String,String>)  {
        
        todo!();
        /*
            const bool rootNoteSet = rootNote != 0;

                setBoolFlag (metadata, AiffAudioFormat::appleOneShot, oneShot == 2);
                setBoolFlag (metadata, AiffAudioFormat::appleRootSet, rootNoteSet);

                if (rootNoteSet)
                    metadata.emplace (AiffAudioFormat::appleRootNote,   String (rootNote));

                metadata.emplace (AiffAudioFormat::appleBeats,          String (numBeats));
                metadata.emplace (AiffAudioFormat::appleDenominator,    String (timeSigDen));
                metadata.emplace (AiffAudioFormat::appleNumerator,      String (timeSigNum));

                const char* keyString = nullptr;

                switch (key)
                {
                    case minor:     keyString = "minor";   break;
                    case major:     keyString = "major";   break;
                    case neither:   keyString = "neither"; break;
                    case both:      keyString = "both";    break;
                    default:                               break;
                }

                if (keyString != nullptr)
                    metadata.emplace (AiffAudioFormat::appleKey, keyString);
        */
    }
    
    pub fn set_bool_flag(&self, 
        values:        &mut HashMap<String,String>,
        name:          *const u8,
        should_be_set: bool)  {
        
        todo!();
        /*
            values.emplace (name, shouldBeSet ? "1" : "0");
        */
    }
}
