crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the TreeView.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum TreeViewColourIds
{
    /**
      | A background colour to fill the component
      | with.
      |
      */
    backgroundColourId             = 0x1000500, 

    /**
      | The colour to draw the lines with.
      |
      */
    linesColourId                  = 0x1000501, 

    /**
      | The colour to use for the drag-and-drop
      | target position indicator.
      |
      */
    dragAndDropIndicatorColourId   = 0x1000502, 

    /**
      | The colour to use to fill the background
      | of any selected items.
      |
      */
    selectedItemBackgroundColourId = 0x1000503, 

    /**
      | The colour to use to fill the background
      | of the odd numbered items.
      |
      */
    oddItemsColourId               = 0x1000504, 

    /**
      | The colour to use to fill the background
      | of the even numbered items.
      |
      */
    evenItemsColourId              = 0x1000505,  
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide
  | 
  | TreeView drawing functionality.
  |
  */
pub trait TreeViewLookAndFeelMethods {

    fn draw_treeview_plus_minus_box(&mut self, 
            _0:                &mut Graphics,
            area:              &Rectangle<f32>,
            background_colour: Colour,
            is_item_open:      bool,
            is_mouse_over:     bool);

    fn are_lines_drawn_for_tree_view(&mut self, _0: &mut TreeView) -> bool;

    fn get_tree_view_indent_size(&mut self, _0: &mut TreeView) -> i32;
}
