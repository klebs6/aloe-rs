crate::ix!();

lazy_static!{
    /*
    template <typename It>
        using ParameterLayoutValidIfIterator = decltype (std::next (std::declval<It>()));
    */
}

//TODO
pub struct ParameterStorageBase {}

/**
  | A class to contain a set of RangedAudioParameters
  | and AudioProcessorParameterGroups
  | containing RangedAudioParameters.
  | 
  | This class is used in the AudioProcessorValueTreeState
  | constructor to allow arbitrarily grouped
  | RangedAudioParameters to be passed
  | to an AudioProcessor.
  |
  */
pub struct AudioProcessorValueTreeStateParameterLayout {
    parameters: Vec<Box<ParameterStorageBase>>,
}

impl AudioProcessorValueTreeStateParameterLayout {

    pub fn new_from_box<Items>(items: Box<Items>) -> Self {
    
        todo!();
        /*
            add (std::move (items)...);
        */
    }
    
    pub fn new_from_range<It: Iterator>(
        begin: It,
        end:   It) -> Self {
    
        todo!();
        /*
            add (begin, end);
        */
    }
    
    pub fn add_from_box<Items>(&mut self, items: Box<Items>)  {
    
        todo!();
        /*
            parameters.reserve (parameters.size() + sizeof... (items));

                // We can replace this with some nicer code once generic lambdas become available. A
                // sequential context like an array initialiser is required to ensure we get the correct
                // order from the parameter pack.
                int unused[] { (parameters.emplace_back (AudioProcessorValueTreeStateParameterLayoutMakeContents() (std::move (items))), 0)... };
                ignoreUnused (unused);
        */
    }
    
    pub fn add_from_range<It: Iterator>(&mut self, 
        begin: It,
        end:   It)  {
    
        todo!();
        /*
            parameters.reserve (parameters.size() + std::size_t (std::distance (begin, end)));
                std::transform (std::make_move_iterator (begin),
                                std::make_move_iterator (end),
                                std::back_inserter (parameters),
                                AudioProcessorValueTreeStateParameterLayoutMakeContents());
        */
    }
    
    pub fn new(other: AudioProcessorValueTreeStateParameterLayout) -> Self {
    
        todo!();
        /*
            swap (other);
        */
    }
    
    pub fn assign_from(&mut self, other: AudioProcessorValueTreeStateParameterLayout) -> &mut AudioProcessorValueTreeStateParameterLayout {
        
        todo!();
        /*
            swap (other); return *this;
        */
    }
    
    pub fn swap(&mut self, other: &mut AudioProcessorValueTreeStateParameterLayout)  {
        
        todo!();
        /*
            std::swap (other.parameters, parameters);
        */
    }
    
    pub fn add(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
