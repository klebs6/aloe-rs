crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the combo
  | box.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | To change the colours of the menu that
  | pops up, you can set the colour IDs in
  | typename PopupMenu::ColourIDs.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum ComboBoxColourIds
{
    /**
      | The background colour to fill the box
      | with.
      |
      */
    backgroundColourId     = 0x1000b00,   

    /**
      | The colour for the text in the box.
      |
      */
    textColourId           = 0x1000a00,   

    /**
      | The colour for an outline around the
      | box.
      |
      */
    outlineColourId        = 0x1000c00,   

    /**
      | The base colour for the button (a LookAndFeel
      | class will probably use variations
      | on this).
      |
      */
    buttonColourId         = 0x1000d00,   

    /**
      | The colour for the arrow shape that pops
      | up the menu
      |
      */
    arrowColourId          = 0x1000e00,   

    /**
      | The colour that will be used to draw a
      | box around the edge of the component
      | when it has focus.
      |
      */
    focusedOutlineColourId = 0x1000f00,    
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide
  | 
  | ComboBox functionality.
  |
  */
pub trait ComboBoxLookAndFeelMethods {

    fn draw_combo_box(
        &mut self, 
        _0:             &mut Graphics,
        width:          i32,
        height:         i32,
        is_button_down: bool,
        buttonx:        i32,
        buttony:        i32,
        buttonw:        i32,
        buttonh:        i32,
        _8:             &mut ComboBox
    );

    fn get_combo_box_font(
        &mut self, 
        _0: &mut ComboBox
    ) -> Font;

    fn create_combo_box_text_box(
        &mut self, 
        _0: &mut ComboBox
    ) -> *mut Label;

    fn position_combo_box_text(
        &mut self, 
        _0:                &mut ComboBox,
        label_to_position: &mut Label
    );

    fn get_options_for_combo_box_popup_menu(
        &mut self, 
        _0: &mut ComboBox,
        _1: &mut Label
    ) -> PopupMenuOptions;

    fn draw_combo_box_text_when_nothing_selected(
        &mut self, 
        _0: &mut Graphics,
        _1: &mut ComboBox,
        _2: &mut Label
    );
}
