crate::ix!();

pub trait ContainsLayout {

    /**
      | Returns true if the channel layout map
      | contains a certain layout.
      | 
      | You can use this method to help you implement
      | the checkBusesLayoutSupported method.
      | For example
      | 
      | -----------
      | @code
      | 
      | bool checkBusesLayoutSupported (const AudioProcessorBusesLayout& layouts) override
      | {
      |     return containsLayout (layouts, {{1,1},{2,2}});
      | }
      |
      */
    fn contains_layout(
        layouts:             &dyn AudioProcessorBusesLayoutInterface,
        channel_layout_list: &[ [i16; 2] ]) -> bool where Self: Sized;

    fn contains_layouts<const numLayouts: usize>(
        layouts:             &dyn AudioProcessorBusesLayoutInterface,
        channel_layout_list: &[[i16; numLayouts]; 2]) -> bool where Self: Sized;
}

pub trait GetNextBestLayoutInLayoutList {

    /**
      | Returns the next best layout which is
      | contained in a channel layout map.
      | 
      | You can use this method to help you implement
      | getNextBestLayout. For example:
      | 
      | -----------
      | @code
      | 
      | AudioProcessorBusesLayout getNextBestLayout (const AudioProcessorBusesLayout& layouts) override
      | {
      |     return getNextBestLayoutInLayoutList (layouts, {{1,1},{2,2}});
      | }
      |
      */
    fn get_next_best_layout_in_layout_list(
        &mut self, 
        layouts:             &dyn AudioProcessorBusesLayoutInterface,
        channel_layout_list: &[&[i16]; 2]) -> Box<dyn AudioProcessorBusesLayoutInterface>;
}
