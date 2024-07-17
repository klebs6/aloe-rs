crate::ix!();

/**
  | Stores the last N messages. Safe to access
  | from the message thread only.
  |
  */
pub struct MidiListModel {
    on_change: fn() -> (),
    messages:  Vec<MidiMessage>,
}

pub const MIDI_LIST_MODEL_NUM_TO_STORE: usize = 1000;

impl Index<usize> for MidiListModel {

    type Output = MidiMessage;
    
    #[inline] fn index(&self, ind: usize) -> &Self::Output {
        todo!();
        /*
            return messages[ind];
        */
    }
}

impl MidiListModel {

    pub fn add_messages<It>(&mut self, 
        begin: It,
        end:   It)  {
    
        todo!();
        /*
            const auto numNewMessages = (int) std::distance (begin, end);
            const auto numToAdd = jmin (numToStore, numNewMessages);
            const auto numToRemove = jmax (0, (int) messages.size() + numToAdd - numToStore);
            messages.erase (messages.begin(), std::next (messages.begin(), numToRemove));
            messages.insert (messages.end(), std::prev (end, numToAdd), end);

            if (onChange != nullptr)
                onChange();
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            messages.clear();

            if (onChange != nullptr)
                onChange();
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return messages.size();
        */
    }
}
