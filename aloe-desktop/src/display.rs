crate::ix!();

/**
  | Represents a connected display device.
  |
  */
pub struct Display {

    /**
      | This will be true if this is the user's
      | main display device.
      |
      */
    is_main:           bool,

    /**
      | The total area of this display in logical pixels
      | including any OS-dependent objects like the taskbar,
      | menu bar, etc.
      */
    total_area:        Rectangle<i32>,

    /**
      | The total area of this display in logical pixels
      | which isn't covered by OS-dependent objects like
      | the taskbar, menu bar, etc.
      */
    user_area:         Rectangle<i32>,

    /**
      | Represents the area of this display in logical
      | pixels that is not functional for displaying content.
      | On mobile devices this may be the area covered
      | by display cutouts and notches, where you still
      | want to draw a background but should not position
      | important content.
      */
    safe_area_insets:  BorderSize<i32>,

    /**
      | The top-left of this display in physical
      | coordinates.
      |
      */
    top_left_physical: Point<i32>,

    /**
      | The scale factor of this display. For higher-resolution
      | displays, or displays with a user-defined scale
      | factor set, this may be a value other than 1.0.
      | This value is used to convert between physical
      | and logical pixels. For example, a Component with
      | size 10x10 will use 20x20 physical pixels on a
      | display with a scale factor of 2.0.
      */
    scale:             f64,

    /**
      | The DPI of the display. This is the number of physical
      | pixels per inch. To get the number of logical pixels
      | per inch, divide this by the Display::scale value.
      */
    dpi:               f64,
}
