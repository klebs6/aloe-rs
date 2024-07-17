crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide button-drawing
  | functionality.
  |
  */
pub trait ButtonLookAndFeelMethods {

    fn draw_button_background(
        &mut self, 
        _0:                                &mut Graphics,
        _1:                                &mut Button,
        background_colour:                 &Colour,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool
    );

    fn get_text_button_font(
        &mut self, 
        _0:            &mut TextButton,
        button_height: i32
    ) -> Font;

    fn get_text_button_width_to_fit_text(
        &mut self, 
        _0:            &mut TextButton,
        button_height: i32
    ) -> i32;

    /**
      | Draws the text for a TextButton.
      |
      */
    fn draw_button_text(
        &mut self, 
        _0:                                &mut Graphics,
        _1:                                &mut TextButton,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool
    );

    /**
      | Draws the contents of a standard ToggleButton.
      |
      */
    fn draw_toggle_button(
        &mut self, 
        _0:                                &mut Graphics,
        _1:                                &mut ToggleButton,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool
    );

    fn change_toggle_button_width_to_fit_text(&mut self, _0: &mut ToggleButton);

    fn draw_tick_box(
        &mut self, 
        _0:                                &mut Graphics,
        _1:                                &mut Component,
        x:                                 f32,
        y:                                 f32,
        w:                                 f32,
        h:                                 f32,
        ticked:                            bool,
        is_enabled:                        bool,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool
    );

    fn draw_drawable_button(
        &mut self, 
        _0:                                &mut Graphics,
        _1:                                &mut DrawableButton,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool
    );
}
