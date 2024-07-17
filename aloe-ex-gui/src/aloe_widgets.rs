crate::ix!();

/**
  | The top-level component containing
  | the accessible Aloe widget examples.
  | 
  | Most Aloe UI elements have built-in
  | accessibility support and will be visible
  | and controllable by accessibility
  | clients. There are a few examples of
  | some widgets in this demo such as Sliders,
  | Buttons and a TreeView.
  |
  */
#[no_copy]
#[leak_detector]
pub struct ALOEWidgetsComponent<'a> {
    base:                Component<'a>,
    description_label:   Label<'a>, 
    buttons_component:   ButtonsComponent<'a>,
    sliders_component:   SlidersComponent<'a>,
    tree_view_component: TreeViewComponent<'a>,
    buttons:             ContentComponent<'a>, //{ "Buttons", "Examples of some Aloe buttons.", buttonsComponent };
    sliders:             ContentComponent<'a>, //{ "Sliders", "Examples of some Aloe sliders.", slidersComponent };
    tree_view:           ContentComponent<'a>, //{ "TreeView", "A Aloe TreeView.", treeViewComponent };
}

pub fn aloe_widgets_component_default_description_label<'a>() -> Label<'a> {
    todo!();
    /*
       { {}, 
       "This is a demo of a few of the accessible built-in Aloe widgets.\n\n" 
       "To navigate this demo with a screen reader, either enable VoiceOver on macOS and iOS, " 
       "TalkBack on Android, or Narrator on Windows and follow the navigational prompts." };
       */
}

impl<'a> Default for ALOEWidgetsComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setTitle ("Aloe Widgets");
            setDescription ("A demo of a few of the accessible built-in Aloe widgets.");
            setFocusContainerType (FocusContainerType::focusContainer);

            addAndMakeVisible (descriptionLabel);

            addAndMakeVisible (buttons);
            addAndMakeVisible (sliders);
            addAndMakeVisible (treeView)
        */
    }
}

impl<'a> Resized for ALOEWidgetsComponent<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

            grid.templateRows = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (2)) };
            grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)) };

            grid.items = { GridItem (descriptionLabel).withMargin ({ 2 }).withColumn ({ GridItem::Span (2) }),
                           GridItem (buttons).withMargin ({ 2 }),
                           GridItem (sliders).withMargin ({ 2 }),
                           GridItem (treeView).withMargin ({ 2 }).withColumn ({ GridItem::Span (2) }) };

            grid.performLayout (getLocalBounds());
        */
    }
}
