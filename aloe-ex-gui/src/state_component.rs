crate::ix!();

#[no_copy]
#[leak_detector]
pub struct StateComponent<'a> {
    base:       Component<'a>,
    properties: [StateComponentProperty<'a>; 12],
}

impl<'a> Default for StateComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            for (auto& property : properties)
                    addAndMakeVisible (property.button)
        */
    }
}

impl<'a> StateComponent<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

                grid.templateRows = { Grid::TrackInfo (Grid::Fr (1)),
                                      Grid::TrackInfo (Grid::Fr (1)),
                                      Grid::TrackInfo (Grid::Fr (1)),
                                      Grid::TrackInfo (Grid::Fr (1)),
                                      Grid::TrackInfo (Grid::Fr (1)),
                                      Grid::TrackInfo (Grid::Fr (1)) };

                grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)) };

                for (auto& property : properties)
                    grid.items.add (GridItem (property.button));

                grid.performLayout (getLocalBounds());
        */
    }
    
    pub fn get_accessible_state(&self) -> AccessibleState {
        
        todo!();
        /*
            AccessibleState result;

                for (auto& property : properties)
                    if (property.button.getToggleState())
                        result = property.setState (std::move (result));

                return result;
        */
    }
}
