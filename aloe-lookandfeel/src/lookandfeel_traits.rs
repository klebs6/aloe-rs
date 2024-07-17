crate::ix!();

pub trait GetTypefaceForFont {

    /**
      | Returns the typeface that should be
      | used for a given font.
      | 
      | The default implementation just does
      | what you'd expect it to, but you can override
      | this if you want to intercept fonts and
      | use your own custom typeface object.
      | @see setDefaultTypeface
      |
      */
    fn get_typeface_for_font(&mut self, _0: &Font) -> TypefacePtr;
}

pub trait GetMouseCursorFor {

    /**
      | Override this to get the chance to swap
      | a component's mouse cursor for a customised
      | one.
      |
      */
    fn get_mouse_cursor_for(&mut self, _0: &mut Component) -> MouseCursor;
}

pub trait CreateGraphicsContext {

    /**
      | Creates a new graphics context object.
      |
      */
    fn create_graphics_context(&mut self, 
        image_to_render_on: &Image,
        origin:             Point<i32>,
        initial_clip:       &RectangleList<i32>) -> Box<dyn LowLevelGraphicsContext>;
}

pub trait DrawSpinningWaitAnimation {

    /**
      | Draws a small image that spins to indicate
      | that something's happening.
      | 
      | This method should use the current time
      | to animate itself, so just keep repainting
      | it every so often.
      |
      */
    fn draw_spinning_wait_animation(&mut self, 
        _0:     &mut Graphics,
        colour: &Colour,
        x:      i32,
        y:      i32,
        w:      i32,
        h:      i32);

}

pub trait GetTickShape {

    /**
      | Returns a tick shape for use in yes/no
      | boxes, etc.
      |
      */
    fn get_tick_shape(&mut self, height: f32) -> Path;
}

pub trait GetCrossShape {

    /**
      | Returns a cross shape for use in yes/no
      | boxes, etc.
      |
      */
    fn get_cross_shape(&mut self, height: f32) -> Path;
}

pub trait CreateDropShadowerForComponent {

    fn create_drop_shadower_for_component(&mut self, _0: *mut Component) -> *mut DropShadower;

}

pub trait PlayAlertSound {

    /**
      | Plays the system's default 'beep' noise,
      | to alert the user about something very
      | important.
      |
      */
    fn play_alert_sound(&mut self);
}

/**
  | This class is used to hold a few look and
  | feel base classes which are associated
  | with classes that may not be present
  | because they're from modules other
  | than aloe_gui_basics.
  | 
  | @tags{GUI}
  |
  */
pub mod extra_look_and_feel_traits {

    use super::*;

    /**
      | This abstract base class is implemented
      | by LookAndFeel classes.
      |
      */
    pub trait LassoComponentMethods {
        fn draw_lasso(&mut self, 
            _0:         &mut Graphics,
            lasso_comp: &mut Component);

    }

    /**
      | This abstract base class is implemented
      | by LookAndFeel classes.
      |
      */
    pub trait KeyMappingEditorComponentMethods {
        fn draw_keymap_change_button(&mut self, 
            _0:              &mut Graphics,
            width:           i32,
            height:          i32,
            _3:              &mut Button,
            key_description: &String);
    }

    /**
      | This abstract base class is implemented
      | by LookAndFeel classes.
      |
      */
    pub trait AudioDeviceSelectorComponentMethods {
        fn draw_level_meter(&mut self, 
            _0:     &mut Graphics,
            width:  i32,
            height: i32,
            level:  f32);

    }
}
