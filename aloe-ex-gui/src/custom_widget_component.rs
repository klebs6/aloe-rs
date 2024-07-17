crate::ix!();

pub fn custom_widget_component_default_description_label<'a>() -> Label<'a> {

    todo!();

    /*
       { {}, "This is a demo of a custom accessible widget.\n\n"
       "The Aloe logo component at the bottom of the page will use the settable properties when queried by "
       "an accessibility client." }
       */
}

pub fn custom_widget_component_default_info_compoennt<'a>() -> ContentComponent<'a> {

    todo!();

    /*
       { "Info",
       "Set the title, role, description and help text properties of the component.",
       infoComponent }
       */
}

pub fn custom_widget_component_default_actions_component<'a>() -> ContentComponent<'a> {

    todo!();

    /*
       { "Actions",
       "Specify the accessibility actions that the component can perform. When invoked the indicator will flash.",
       actionsComponent }
       */
}

pub fn custom_widget_component_default_value_interface_component<'a>() -> ContentComponent<'a> {

    todo!();

    /*
       { "Value",
       "Sets the value that this component represents. This can be numeric, ranged or textual and can optionally be read-only.",
       valueInterfaceComponent }
       */
}

pub fn custom_widget_component_default_state_component<'a>() -> ContentComponent<'a> {

    todo!();

    /*
       { "State",
       "Modify the AccessibleState properties of the component.",
       stateComponent }
       */
}

//-------------------------------------------[.cpp/Aloe/examples/GUI/AccessibilityDemo.h]

/**
  | The top-level component containing
  | a customisable accessible widget.
  | 
  | The AccessibleComponent class just
  | draws a Aloe logo and overrides the Component::createAccessibilityHandler()
  | method to return a custom AccessibilityHandler.
  | The properties of this handler are set
  | by the various controls in the demo.
  |
  */
#[no_copy]
#[leak_detector]
pub struct CustomWidgetComponent<'a> {
    base:                      Component<'a>,
    description_label:         Label<'a>,
    info_component:            InfoComponent<'a>,           //{ *this };
    actions_component:         ActionsComponent<'a>,        //{ *this };
    value_interface_component: ValueInterfaceComponent<'a>, //{ *this };
    state_component:           StateComponent<'a>,
    info:                      ContentComponent<'a>,
    actions:                   ContentComponent<'a>,
    value_interface:           ContentComponent<'a>,
    state:                     ContentComponent<'a>,
    accessible_component:      AccessibleComponent<'a>,     //{ *this };
}

impl<'a> Default for CustomWidgetComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setTitle ("Custom Widget");
            setDescription ("A demo of a customisable accessible widget.");
            setFocusContainerType (FocusContainerType::focusContainer);

            addAndMakeVisible (descriptionLabel);

            addAndMakeVisible (infoComponent);
            addAndMakeVisible (actionsComponent);
            addAndMakeVisible (valueInterfaceComponent);
            addAndMakeVisible (stateComponent);
            addAndMakeVisible (accessibleComponent)
        */
    }
}

impl<'a> Resized for CustomWidgetComponent<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

            grid.templateRows = { Grid::TrackInfo (Grid::Fr (1)),
                                  Grid::TrackInfo (Grid::Fr (2)),
                                  Grid::TrackInfo (Grid::Fr (2)),
                                  Grid::TrackInfo (Grid::Fr (2)) };

            grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)) };

            grid.items = { GridItem (descriptionLabel).withMargin ({ 2 }).withColumn ({ GridItem::Span (2) }),
                           GridItem (infoComponent).withMargin ({ 2 }).withColumn ({ GridItem::Span (2) }),
                           GridItem (actionsComponent).withMargin ({ 2 }),
                           GridItem (valueInterfaceComponent).withMargin ({ 2 }),
                           GridItem (stateComponent).withMargin ({ 2 }),
                           GridItem (accessibleComponent).withMargin ({ 10 }) };

            grid.performLayout (getLocalBounds());
        */
    }
}
