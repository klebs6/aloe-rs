crate::ix!();

/**
  | The main demo content component.
  | 
  | This just contains a TabbedComponent
  | with a tab for each of the top-level demos.
  |
  */
#[no_copy]
#[leak_detector]
pub struct AccessibilityDemo<'a> {
    base:                        Component<'a>,
    tooltip_window:              TooltipWindow<'a>, //{ nullptr, 100 };
    tabs:                        TabbedComponent<'a>, //{ TabbedButtonBar::Orientation::TabsAtTop };
    aloe_widgets_component:      ALOEWidgetsComponent<'a>,
    custom_widget_component:     CustomWidgetComponent<'a>,
    custom_navigation_component: CustomNavigationComponent<'a>,
    announcements_component:     AnnouncementsComponent<'a>,
}

impl<'a> Default for AccessibilityDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setTitle ("Accessibility Demo");
            setDescription ("A demo of Aloe's accessibility features.");
            setFocusContainerType (FocusContainerType::focusContainer);

            tabs.setTitle ("Demo tabs");

            const auto tabColour = getLookAndFeel().findColour (ResizableWindow::backgroundColourId).darker (0.1f);

            tabs.addTab ("Aloe Widgets",      tabColour, &aloeWidgetsComponent,      false);
            tabs.addTab ("Custom Widget",     tabColour, &customWidgetComponent,     false);
            tabs.addTab ("Custom Navigation", tabColour, &customNavigationComponent, false);
            tabs.addTab ("Announcements",     tabColour, &announcementsComponent,    false);
            addAndMakeVisible (tabs);

            setSize (800, 600)
        */
    }
}

impl<'a> Paint for AccessibilityDemo<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        */
    }
}

impl<'a> Resized for AccessibilityDemo<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            tabs.setBounds (getLocalBounds());
        */
    }
}
