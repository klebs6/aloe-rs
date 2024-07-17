crate::ix!();

/**
  | The top-level component containing
  | an example of custom child component
  | navigation.
  |
  */
#[no_copy]
#[leak_detector]
pub struct CustomNavigationComponent<'a> {
    base:                 Component<'a>,
    description_label:    Label<'a>,
    navigable_components: NavigableComponentsHolder<'a>,
}

pub fn custom_navigation_component_default_description_label<'a>() -> Label<'a> {
    todo!();

    /*
       { {}, "This is a demo of how to control the navigation order of components when navigating with an accessibility client.\n\n"
       "You can set the order of navigation, whether components are focusable and set a default component which will "
       "receive the focus first." }
       */
}

impl<'a> Default for CustomNavigationComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setTitle ("Custom Navigation");
            setDescription ("A demo of custom component navigation.");
            setFocusContainerType (FocusContainerType::focusContainer);

            addAndMakeVisible (descriptionLabel);
            addAndMakeVisible (navigableComponents)
        */
    }
}

impl<'a> Resized for CustomNavigationComponent<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

            grid.templateRows = { Grid::TrackInfo (Grid::Fr (1)),
                                  Grid::TrackInfo (Grid::Fr (2)) };

            grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)) };

            grid.items = { GridItem (descriptionLabel).withMargin (2),
                           GridItem (navigableComponents).withMargin (5) };

            grid.performLayout (getLocalBounds());
        */
    }
}
