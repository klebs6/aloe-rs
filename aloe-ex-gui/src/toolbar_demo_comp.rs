crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/WidgetsDemo.h]

pub struct ToolbarDemoComp<'a> {
    base:               Component<'a>,
    toolbar:            Toolbar<'a>,
    depth_slider:       Slider<'a>,     // default = { Slider::LinearHorizontal, Slider::TextBoxLeft  }
    depth_label:        Label<'a>,      // { {}, "Toolbar depth:" },
    info_label:         Label<'a>,      // { {}, "As well as showing off toolbars, this demo illustrates how to store " "a set of SVG files in a Zip file, embed that in your application, and read " "them back in at runtime.\n\nThe icon images here are taken from the open-source " "Tango icon project."};
    orientation_button: TextButton<'a>, // default = { "Vertical/Horizontal"  }
    customise_button:   TextButton<'a>, // default = { "Customise..."  }
    factory:            DemoToolbarItemFactory<'a>,
}

impl<'a> SliderListener for ToolbarDemoComp<'a> {

}

impl<'a> SliderValueChanged for ToolbarDemoComp<'a> {

    fn slider_value_changed(&mut self, _0: *mut Slider)  {
        
        todo!();
        /*
            resized();
        */
    }
}

impl<'a> SliderDragStarted for ToolbarDemoComp<'a> {
    fn slider_drag_started(&mut self, _: *mut aloe_slider::Slider<'_>) { todo!() }
}

impl<'a> SliderDragEnded for ToolbarDemoComp<'a> {
    fn slider_drag_ended(&mut self, _: *mut aloe_slider::Slider<'_>) { todo!() }
}

impl<'a> Default for ToolbarDemoComp<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            // Create and add the toolbar...
            addAndMakeVisible (toolbar);

            // And use our item factory to add a set of default icons to it...
            toolbar.addDefaultItems (factory);

            // Now we'll just create the other sliders and buttons on the demo page, which adjust
            // the toolbar's properties...
            addAndMakeVisible (infoLabel);
            infoLabel.setJustificationType (Justification::topLeft);
            infoLabel.setBounds (80, 80, 450, 100);
            infoLabel.setInterceptsMouseClicks (false, false);

            addAndMakeVisible (depthSlider);
            depthSlider.setRange (10.0, 200.0, 1.0);
            depthSlider.setValue (50, dontSendNotification);
            depthSlider.addListener (this);
            depthSlider.setBounds (80, 210, 300, 22);
            depthLabel.attachToComponent (&depthSlider, false);

            addAndMakeVisible (orientationButton);
            orientationButton.onClick = [this] { toolbar.setVertical (! toolbar.isVertical()); resized(); };
            orientationButton.changeWidthToFitText (22);
            orientationButton.setTopLeftPosition (depthSlider.getX(), depthSlider.getBottom() + 20);

            addAndMakeVisible (customiseButton);
            customiseButton.onClick = [this] { toolbar.showCustomisationDialog (factory); };
            customiseButton.changeWidthToFitText (22);
            customiseButton.setTopLeftPosition (orientationButton.getRight() + 20, orientationButton.getY())
        */
    }
}

impl<'a> Resized for ToolbarDemoComp<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            auto toolbarThickness = (int) depthSlider.getValue();

            if (toolbar.isVertical())
                toolbar.setBounds (getLocalBounds().removeFromLeft (toolbarThickness));
            else
                toolbar.setBounds (getLocalBounds().removeFromTop  (toolbarThickness));
        */
    }
}
