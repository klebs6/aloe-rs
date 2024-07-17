crate::ix!();

pub fn get_channel_name(
        buses: &Vec<Box<AudioProcessorBus>>,
        index: i32) -> String {
    
    todo!();
        /*
            return buses.size() > 0 ? AudioChannelSet::getChannelTypeName (buses[0]->getCurrentLayout().getTypeOfChannel (index)) : String();
        */
}

pub fn is_stereo_pair(
        buses: &Vec<Box<AudioProcessorBus>>,
        index: i32) -> bool {
    
    todo!();
        /*
            return index < 2
                && buses.size() > 0
                && buses[0]->getCurrentLayout() == AudioChannelSet::stereo();
        */
}
