crate::ix!();

/**
  | This component lives inside our window,
  | and this is where you should put all your
  | controls and content.
  |
  */
#[no_copy]
#[leak_detector]
pub struct ComponentDemo<'a> {
    base:       Component<'a>,
    light_grid: ToggleLightGridComponent<'a>,
}

impl<'a> Default for ComponentDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            // add the light grid to out main component.
            addAndMakeVisible (lightGrid);

            setSize (600, 600)
        */
    }
}

impl<'a> ComponentDemo<'a> {

    pub fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // set the size of the grid to fill the whole window.
            lightGrid.setBounds (getLocalBounds());
        */
    }
}
