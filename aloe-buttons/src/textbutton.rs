crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_TextButton.h]

/**
  | A button that uses the standard lozenge-shaped
  | background with a line of text on it.
  | 
  | @see Button, DrawableButton
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TextButton<'a> {
    base: Button<'a>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_TextButton.cpp]
impl<'a> From<&str> for TextButton<'a> {

    /**
      | Creates a TextButton.
      | 
      | -----------
      | @param buttonName
      | 
      | the text to put in the button (the component's
      | name is also initially set to this string,
      | but these can be changed later using
      | the setName() and setButtonText()
      | methods)
      |
      */
    fn from(name: &str) -> Self {
    
        todo!();
        /*
        : button(name),

        
        */
    }
}

impl<'a> TextButton<'a> {

    /**
      | Creates a TextButton.
      | 
      | -----------
      | @param buttonName
      | 
      | the text to put in the button (the component's
      | name is also initially set to this string,
      | but these can be changed later using
      | the setName() and setButtonText()
      | methods)
      | ----------
      | @param toolTip
      | 
      | an optional string to use as a tooltip
      |
      */
    pub fn new(
        name:     &String,
        tool_tip: &String) -> Self {
    
        todo!();
        /*
        : button(name),

            setTooltip (toolTip);
        */
    }
}

impl<'a> Default for TextButton<'a> {
    
    /**
      | Creates a TextButton.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : button(String()),

        
        */
    }
}

impl<'a> PaintButton for TextButton<'a> {

    fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();

        lf.drawButtonBackground (g, *this,
                                 findColour (getToggleState() ? buttonOnColourId : buttonColourId),
                                 shouldDrawButtonAsHighlighted, shouldDrawButtonAsDown);

        lf.drawButtonText (g, *this, shouldDrawButtonAsHighlighted, shouldDrawButtonAsDown);
        */
    }
}

impl<'a> ColourChanged for TextButton<'a> {
    
    fn colour_changed(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
}

impl<'a> ChangeWidthToFitText for TextButton<'a> {

    /**
      | Changes this button's width to fit neatly
      | around its current text, without changing
      | its height.
      |
      */
    fn change_width_to_fit_text(&mut self)  {
        
        todo!();
        /*
            changeWidthToFitText (getHeight());
        */
    }

    /**
      | Resizes the button's width to fit neatly
      | around its current text, and gives it
      | the specified height.
      |
      */
    fn change_width_to_fit_text_with_height(&mut self, new_height: i32)  {
        
        todo!();
        /*
            setSize (getBestWidthForHeight (newHeight), newHeight);
        */
    }
}

impl<'a> GetBestWidthForHeight for TextButton<'a> {

    /**
      | Returns the width that the LookAndFeel
      | suggests would be best for this button
      | if it had the given height.
      |
      */
    fn get_best_width_for_height(&mut self, button_height: i32) -> i32 {
        
        todo!();
        /*
            return getLookAndFeel().getTextButtonWidthToFitText (*this, buttonHeight);
        */
    }
}
