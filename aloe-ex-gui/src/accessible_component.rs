crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AccessibleComponent<'a> {
    base:                    Component<'a>,
    custom_widget_component: &'a mut CustomWidgetComponent<'a>,
    aloe_logo_path:          Path, //{ getALOELogoPath() };
}

impl<'a> AccessibleComponent<'a> {

    pub fn new(owner: &mut CustomWidgetComponent) -> Self {
    
        todo!();
        /*
        : custom_widget_component(owner),

        
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (Colours::darkorange);

                g.fillPath (aloeLogoPath,
                            aloeLogoPath.getTransformToScaleToFit (getLocalBounds().toFloat(), true));
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            /**
                    The AccessibilityHandler class is the interface between Aloe components
                    and accessibility clients. This derived class represents the properties
                    set via the demo UI.
                */
                struct CustomAccessibilityHandler  : public AccessibilityHandler
                {
                    explicit CustomAccessibilityHandler (CustomWidgetComponent& comp)
                        : AccessibilityHandler (comp.accessibleComponent,
                                                comp.infoComponent.getRole(),
                                                comp.actionsComponent.getActions(),
                                                { comp.valueInterfaceComponent.getValueInterface() }),
                          customWidgetComponent (comp)
                    {
                    }

                    String getTitle() const override                  { return customWidgetComponent.infoComponent.getTitle(); }
                    String getDescription() const override            { return customWidgetComponent.infoComponent.getDescription(); }
                    String getHelp() const override                   { return customWidgetComponent.infoComponent.getHelp(); }

                    AccessibleState getCurrentState() const override  { return customWidgetComponent.stateComponent.getAccessibleState(); }

                    CustomWidgetComponent& customWidgetComponent;
                };

                return std::make_unique<CustomAccessibilityHandler> (customWidgetComponent);
        */
    }
}
