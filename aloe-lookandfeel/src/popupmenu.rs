crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the menu.
  | 
  | These constants can be used either via
  | the LookAndFeel::setColour() method
  | for the look and feel that is set for this
  | menu with setLookAndFeel()
  | 
  | @see setLookAndFeel, LookAndFeel::setColour,
  | LookAndFeel::findColour
  |
  */
pub enum PopupMenuColourIds
{
    /**
      | The colour to fill the menu's background
      | with.
      |
      */
    backgroundColourId             = 0x1000700,  

    /**
      | The colour for normal menu item text,
      | (unless the colour is specified when
      | the item is added).
      |
      */
    textColourId                   = 0x1000600,  

    /**
      | The colour for section header item text
      | (see the addSectionHeader() method).
      |
      */
    headerTextColourId             = 0x1000601,  

    /**
      | The colour to fill the background of
      | the currently highlighted menu item.
      |
      */
    highlightedBackgroundColourId  = 0x1000900,  

    /**
      | The colour to use for the text of the currently
      | highlighted item.
      |
      */
    highlightedTextColourId        = 0x1000800,  

}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide menu
  | drawing functionality.
  |
  */
pub trait PopupMenuLookAndFeelMethods {

    /**
      | Fills the background of a popup menu
      | component.
      |
      */
    fn draw_popup_menu_background(
        &mut self, 
        _0:     &mut Graphics,
        width:  i32,
        height: i32
    ) {}

    /**
      | Fills the background of a popup menu
      | component.
      |
      */
    fn draw_popup_menu_background_with_options(
        &mut self, 
        _0:     &mut Graphics,
        width:  i32,
        height: i32,
        _3:     &PopupMenuOptions
    );

    /**
      | Draws one of the items in a popup menu.
      |
      */
    fn draw_popup_menu_item(
        &mut self, 
        _0:                &mut Graphics,
        area:              &Rectangle<i32>,
        is_separator:      bool,
        is_active:         bool,
        is_highlighted:    bool,
        is_ticked:         bool,
        has_sub_menu:      bool,
        text:              &String,
        shortcut_key_text: &String,
        icon:              *const Drawable,
        text_colour:       *const Colour
    ) {}

    /**
      | Draws one of the items in a popup menu.
      |
      */
    fn draw_popup_menu_item_with_options(
        &mut self, 
        _0:             &mut Graphics,
        area:           &Rectangle<i32>,
        is_highlighted: bool,
        item:           &PopupMenuItem,
        _4:             &PopupMenuOptions
    );

    fn draw_popup_menu_section_header(
        &mut self, 
        _0: &mut Graphics,
        _1: &Rectangle<i32>,
        _2: &String

    ) {}

    fn draw_popup_menu_section_header_with_options(
        &mut self, 
        _0:           &mut Graphics,
        area:         &Rectangle<i32>,
        section_name: &String,
        _3:           &PopupMenuOptions
    );

    /**
      | Returns the size and style of font to
      | use in popup menus.
      |
      */
    fn get_popup_menu_font(&mut self) -> Font;

    fn draw_popup_menu_up_down_arrow(
        &mut self, 
        _0:                 &mut Graphics,
        width:              i32,
        height:             i32,
        is_scroll_up_arrow: bool
    ) {}

    fn draw_popup_menu_up_down_arrow_with_options(
        &mut self, 
        _0:                 &mut Graphics,
        width:              i32,
        height:             i32,
        is_scroll_up_arrow: bool,
        _4:                 &PopupMenuOptions
    );

    /**
      | Finds the best size for an item in a popup
      | menu.
      |
      */
    fn get_ideal_popup_menu_item_size(
        &mut self, 
        text:                      &String,
        is_separator:              bool,
        standard_menu_item_height: i32,
        ideal_width:               &mut i32,
        ideal_height:              &mut i32
    ) {}

    /**
      | Finds the best size for an item in a popup
      | menu.
      |
      */
    fn get_ideal_popup_menu_item_size_with_options(
        &mut self, 
        text:                      &String,
        is_separator:              bool,
        standard_menu_item_height: i32,
        ideal_width:               &mut i32,
        ideal_height:              &mut i32,
        _5:                        &PopupMenuOptions
    );

    fn get_menu_window_flags(&mut self) -> i32;

    fn draw_menu_bar_background(
        &mut self, 
        _0:                &mut Graphics,
        width:             i32,
        height:            i32,
        is_mouse_over_bar: bool,
        _4:                &mut MenuBarComponent
    );

    fn get_default_menu_bar_height(&mut self) -> i32;

    fn get_menu_bar_item_width(
        &mut self, 
        _0:         &mut MenuBarComponent,
        item_index: i32,
        item_text:  &String
    ) -> i32;

    fn get_menu_bar_font(
        &mut self, 
        _0:         &mut MenuBarComponent,
        item_index: i32,
        item_text:  &String
    ) -> Font;

    fn draw_menu_bar_item(
        &mut self, 
        _0:                 &mut Graphics,
        width:              i32,
        height:             i32,
        item_index:         i32,
        item_text:          &String,
        is_mouse_over_item: bool,
        is_menu_open:       bool,
        is_mouse_over_bar:  bool,
        _8:                 &mut MenuBarComponent
    );

    fn get_parent_component_for_menu_options(&mut self, options: &PopupMenuOptions) -> *mut Component;

    fn prepare_popup_menu_window(&mut self, new_window: &mut Component);

    /**
      | Return true if you want your popup menus
      | to scale with the target component's
      | AffineTransform or scale factor
      |
      */
    fn should_popup_menu_scale_with_target_component(&mut self, options: &PopupMenuOptions) -> bool;

    fn get_popup_menu_border_size(&mut self) -> i32 { 0 }

    fn get_popup_menu_border_size_with_options(&mut self, _0: &PopupMenuOptions) -> i32;

    /**
      | Implement this to draw some custom decoration
      | between the columns of the popup menu.
      | 
      | `getPopupMenuColumnSeparatorWidthWithPopupMenuOptions`
      | must return a positive value in order
      | to display the separator.
      |
      */
    fn draw_popup_menu_column_separator_with_options(
        &mut self, 
        g:      &mut Graphics,
        bounds: &Rectangle<i32>,
        _2:     &PopupMenuOptions
    );

    /**
      | Return the amount of space that should
      | be left between popup menu columns.
      |
      */
    fn get_popup_menu_column_separator_width_with_options(&mut self, _0: &PopupMenuOptions) -> i32;
}
