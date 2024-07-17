crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/MenusDemo.h]

/**
  | This struct contains a header component
  | that will be used when the burger menu
  | is enabled. It contains an icon that
  | can be used to show the side panel containing
  | the menu.
  |
  */
#[no_copy]
#[leak_detector]
pub struct BurgerMenuHeader<'a> {
    base:          Component<'a>,
    side_panel:    &'a mut SidePanel<'a>,
    title_label:   Label<'a>, // default = { "titleLabel", "Aloe Demo"  }
    burger_button: ShapeButton<'a>, // default = { "burgerButton", Colours::lightgrey, Colours::lightgrey, Colours::white  }
}

impl<'a> Drop for BurgerMenuHeader<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
            sidePanel.showOrHide (false);
         */
    }
}

impl<'a> BurgerMenuHeader<'a> {

    pub fn new(sp: &mut SidePanel) -> Self {
    
        todo!();
        /*
        : side_panel(sp),

            static const unsigned char burgerMenuPathData[]
                = { 110,109,0,0,128,64,0,0,32,65,108,0,0,224,65,0,0,32,65,98,254,212,232,65,0,0,32,65,0,0,240,65,252,
                    169,17,65,0,0,240,65,0,0,0,65,98,0,0,240,65,8,172,220,64,254,212,232,65,0,0,192,64,0,0,224,65,0,0,
                    192,64,108,0,0,128,64,0,0,192,64,98,16,88,57,64,0,0,192,64,0,0,0,64,8,172,220,64,0,0,0,64,0,0,0,65,
                    98,0,0,0,64,252,169,17,65,16,88,57,64,0,0,32,65,0,0,128,64,0,0,32,65,99,109,0,0,224,65,0,0,96,65,108,
                    0,0,128,64,0,0,96,65,98,16,88,57,64,0,0,96,65,0,0,0,64,4,86,110,65,0,0,0,64,0,0,128,65,98,0,0,0,64,
                    254,212,136,65,16,88,57,64,0,0,144,65,0,0,128,64,0,0,144,65,108,0,0,224,65,0,0,144,65,98,254,212,232,
                    65,0,0,144,65,0,0,240,65,254,212,136,65,0,0,240,65,0,0,128,65,98,0,0,240,65,4,86,110,65,254,212,232,
                    65,0,0,96,65,0,0,224,65,0,0,96,65,99,109,0,0,224,65,0,0,176,65,108,0,0,128,64,0,0,176,65,98,16,88,57,
                    64,0,0,176,65,0,0,0,64,2,43,183,65,0,0,0,64,0,0,192,65,98,0,0,0,64,254,212,200,65,16,88,57,64,0,0,208,
                    65,0,0,128,64,0,0,208,65,108,0,0,224,65,0,0,208,65,98,254,212,232,65,0,0,208,65,0,0,240,65,254,212,
                    200,65,0,0,240,65,0,0,192,65,98,0,0,240,65,2,43,183,65,254,212,232,65,0,0,176,65,0,0,224,65,0,0,176,
                    65,99,101,0,0 };

            Path p;
            p.loadPathFromData (burgerMenuPathData, sizeof (burgerMenuPathData));
            burgerButton.setShape (p, true, true, false);

            burgerButton.onClick = [this] { showOrHide(); };
            addAndMakeVisible (burgerButton);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto titleBarBackgroundColour = getLookAndFeel().findColour (ResizableWindow::backgroundColourId)
                                                            .darker();

            g.setColour (titleBarBackgroundColour);
            g.fillRect (getLocalBounds());
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds();

            burgerButton.setBounds (r.removeFromRight (40).withSizeKeepingCentre (20, 20));

            titleLabel.setFont (Font ((float) getHeight() * 0.5f, Font::plain));
            titleLabel.setBounds (r);
        */
    }
    
    pub fn show_or_hide(&mut self)  {
        
        todo!();
        /*
            sidePanel.showOrHide (! sidePanel.isPanelShowing());
        */
    }
}
