crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/GraphicsDemo.h]

/**
  | Holds the various toggle buttons for
  | the animation modes.
  |
  */
#[no_copy]
#[leak_detector]
pub struct ControllersComponent<'a> {
    base:              Component<'a>,
    animate_rotation:  ToggleButton<'a>,
    animate_position:  ToggleButton<'a>,
    animate_alpha:     ToggleButton<'a>,
    animate_size:      ToggleButton<'a>,
    animate_shear:     ToggleButton<'a>,
    clip_to_rectangle: ToggleButton<'a>,
    clip_to_path:      ToggleButton<'a>,
    clip_to_image:     ToggleButton<'a>,
    quality:           ToggleButton<'a>,
}

impl<'a> Default for ControllersComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            initialiseToggle (animatePosition, "Animate Position",  true);
            initialiseToggle (animateRotation, "Animate Rotation",  true);
            initialiseToggle (animateSize,     "Animate Size",      false);
            initialiseToggle (animateShear,    "Animate Shearing",  false);
            initialiseToggle (animateAlpha,    "Animate Alpha",     false);
            initialiseToggle (clipToRectangle, "Clip to Rectangle", false);
            initialiseToggle (clipToPath,      "Clip to Path",      false);
            initialiseToggle (clipToImage,     "Clip to Image",     false);
            initialiseToggle (quality,         "Higher quality image interpolation", false)
        */
    }
}

impl<'a> ControllersComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (4);

            int buttonHeight = 22;

            auto columns = r.removeFromTop (buttonHeight * 4);
            auto col = columns.removeFromLeft (200);

            animatePosition.setBounds (col.removeFromTop (buttonHeight));
            animateRotation.setBounds (col.removeFromTop (buttonHeight));
            animateSize    .setBounds (col.removeFromTop (buttonHeight));
            animateShear   .setBounds (col.removeFromTop (buttonHeight));

            columns.removeFromLeft (20);
            col = columns.removeFromLeft (200);

            animateAlpha   .setBounds (col.removeFromTop (buttonHeight));
            clipToRectangle.setBounds (col.removeFromTop (buttonHeight));
            clipToPath     .setBounds (col.removeFromTop (buttonHeight));
            clipToImage    .setBounds (col.removeFromTop (buttonHeight));

            r.removeFromBottom (6);
            quality.setBounds (r.removeFromTop (buttonHeight));
        */
    }
    
    pub fn initialise_toggle(&mut self, 
        b:    &mut ToggleButton,
        name: *const u8,
        on:   bool)  {
        
        todo!();
        /*
            addAndMakeVisible (b);
            b.setButtonText (name);
            b.setToggleState (on, dontSendNotification);
        */
    }
}

