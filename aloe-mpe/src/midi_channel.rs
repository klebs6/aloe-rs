crate::ix!();

pub struct MpeChannelAssignerMidiChannel
{
    notes:            Vec<i32>,
    last_note_played: i32, // default = -1
}

impl MpeChannelAssignerMidiChannel {
    
    pub fn is_free(&self) -> bool {
        
        todo!();
        /*
            return notes.isEmpty();
        */
    }
}
