crate::ix!();

pub struct PopupMenuLookAndFeel<'a> {
    base: LookAndFeel_V4<'a>,
}

impl<'a> PopupMenuLookAndFeel<'a> {

    pub fn draw_popup_menu_column_separator_with_options(
        &mut self, 
        g:      &mut Graphics,
        bounds: &Rectangle<i32>,
        opt:    &PopupMenuOptions
    ) {
        
        todo!();
        /*
            if (auto* target = opt.getTargetComponent())
                {
                    const auto baseColour = target->findColour (TextButton::buttonColourId);
                    g.setColour (baseColour.brighter (0.4f));

                    const float dashes[] { 5.0f, 5.0f };
                    const auto centre = bounds.toFloat().getCentre();

                    g.drawDashedLine ({ centre.withY ((float) bounds.getY()),
                                        centre.withY ((float) bounds.getBottom()) },
                                      dashes,
                                      numElementsInArray (dashes),
                                      3.0f);
                }
        */
    }
    
    pub fn draw_popup_menu_background_with_options(&mut self, 
        g:   &mut Graphics,
        _1:  i32,
        _2:  i32,
        opt: &PopupMenuOptions)  {
        
        todo!();
        /*
            if (auto* target = opt.getTargetComponent())
                {
                    g.fillAll (target->findColour (TextButton::buttonColourId));
                }
        */
    }

    /**
       Return the amount of space that should be
       left between popup menu columns.
      */
    pub fn get_popup_menu_column_separator_width_with_options(&mut self, _0: &PopupMenuOptions) -> i32 {
        
        todo!();
        /*
            return 10;
        */
    }
}
