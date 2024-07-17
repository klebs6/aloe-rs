crate::ix!();

#[no_copy]
#[leak_detector]
pub struct PropertiesDemo<'a> {
    base:             Component<'a>,
    base2:            Timer,
    concertina_panel: ConcertinaPanel<'a>,
}

impl<'a> Default for PropertiesDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);
            addAndMakeVisible (concertinaPanel);

            {
                auto* panel = new PropertyPanel ("Text Editors");
                panel->addProperties (createTextEditors());
                addPanel (panel);
            }

            {
                auto* panel = new PropertyPanel ("Sliders");
                panel->addSection ("Section 1", createSliders (4), true);
                panel->addSection ("Section 2", createSliders (3), true);
                addPanel (panel);
            }

            {
                auto* panel = new PropertyPanel ("Choice Properties");
                panel->addProperties (createChoices (3));
                addPanel (panel);
            }

            {
                auto* panel = new PropertyPanel ("Buttons & Toggles");
                panel->addProperties (createButtons (6));
                addPanel (panel);
            }

            setSize (750, 650);
            startTimer (300)
        */
    }
}

impl<'a> PropertiesDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colour::greyLevel (0.8f)));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            concertinaPanel.setBounds (getLocalBounds().reduced (4));
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();
            concertinaPanel.expandPanelFully (concertinaPanel.getPanel (0), true);
        */
    }
    
    pub fn add_panel(&mut self, panel: *mut PropertyPanel)  {
        
        todo!();
        /*
            concertinaPanel.addPanel (-1, panel, true);
            concertinaPanel.setMaximumPanelSize (panel, panel->getTotalContentHeight());
        */
    }
}
