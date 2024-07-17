crate::ix!();

pub struct VSTPluginInstanceSpeakerArrangements
{
    in_: *const SpeakerArrangement,
    out: *const SpeakerArrangement,
}

impl VSTPluginInstanceSpeakerArrangements {
    
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return in != nullptr && out != nullptr;
        */
    }
}
