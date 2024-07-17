crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_ColourSelector.h]
pub trait ColourSelectorInterface: 
    GetNumSwatches
    + GetSwatchColour
    + SetSwatchColour {}

pub trait GetNumSwatches {

    /**
      | Tells the selector how many preset colour
      | swatches you want to have on the component.
      | 
      | To enable swatches, you'll need to override
      | getNumSwatches(), getSwatchColour(),
      | and setSwatchColour(), to return the
      | number of colours you want, and to set
      | and retrieve their values.
      |
      */
    fn get_num_swatches(&self) -> i32;
}

pub trait GetSwatchColour {

    /**
      | Called by the selector to find out the
      | colour of one of the swatches.
      | 
      | Your subclass should return the colour
      | of the swatch with the given index.
      | 
      | To enable swatches, you'll need to override
      | getNumSwatches(), getSwatchColour(),
      | and setSwatchColour(), to return the
      | number of colours you want, and to set
      | and retrieve their values.
      |
      */
    fn get_swatch_colour(&self, index: i32) -> Colour;
}

pub trait SetSwatchColour {

    /**
      | Called by the selector when the user
      | puts a new colour into one of the swatches.
      | 
      | Your subclass should change the colour
      | of the swatch with the given index.
      | 
      | To enable swatches, you'll need to override
      | getNumSwatches(), getSwatchColour(),
      | and setSwatchColour(), to return the
      | number of colours you want, and to set
      | and retrieve their values.
      |
      */
    fn set_swatch_colour(&mut self, 
            index:      i32,
            new_colour: &Colour);
}
