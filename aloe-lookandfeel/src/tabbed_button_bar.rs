crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the component.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum TabbedButtonBarColourIds
{
    /**
      | The colour to use to draw an outline around
      | the tabs.
      |
      */
    tabOutlineColourId              = 0x1005812,    

    /**
      | The colour to use to draw the tab names.
      | If this isn't specified, the look and
      | feel will choose an appropriate colour.
      |
      */
    tabTextColourId                 = 0x1005813,    

    /**
      | The colour to use to draw an outline around
      | the currently-selected tab.
      |
      */
    frontOutlineColourId            = 0x1005814,    

    /**
      | The colour to use to draw the currently-selected
      | tab name. If this isn't specified, the
      | look and feel will choose an appropriate
      | colour.
      |
      */
    frontTextColourId               = 0x1005815,    
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide window
  | drawing functionality.
  |
  */
pub trait TabbedButtonBarLookAndFeelMethods {

    fn get_tab_button_space_around_image(&mut self) -> i32;

    fn get_tab_button_overlap(&mut self, tab_depth: i32) -> i32;

    fn get_tab_button_best_width(&mut self, 
        _0:        &mut TabBarButton,
        tab_depth: i32) -> i32;

    fn get_tab_button_extra_component_bounds(&mut self, 
        _0:         &TabBarButton,
        text_area:  &mut Rectangle<i32>,
        extra_comp: &mut Component) -> Rectangle<i32>;

    fn draw_tab_button(&mut self, 
        _0:            &mut TabBarButton,
        _1:            &mut Graphics,
        is_mouse_over: bool,
        is_mouse_down: bool);

    fn get_tab_button_font(&mut self, 
        _0:     &mut TabBarButton,
        height: f32) -> Font;

    fn draw_tab_button_text(&mut self, 
        _0:            &mut TabBarButton,
        _1:            &mut Graphics,
        is_mouse_over: bool,
        is_mouse_down: bool);

    fn draw_tabbed_button_bar_background(&mut self, 
        _0: &mut TabbedButtonBar,
        _1: &mut Graphics);

    fn draw_tab_area_behind_front_button(&mut self, 
        _0: &mut TabbedButtonBar,
        _1: &mut Graphics,
        w:  i32,
        h:  i32);

    fn create_tab_button_shape(&mut self, 
        _0:            &mut TabBarButton,
        path:          &mut Path,
        is_mouse_over: bool,
        is_mouse_down: bool);

    fn fill_tab_button_shape(&mut self, 
        _0:            &mut TabBarButton,
        _1:            &mut Graphics,
        path:          &Path,
        is_mouse_over: bool,
        is_mouse_down: bool);

    fn create_tab_bar_extras_button(&mut self) -> *mut Button;

}

