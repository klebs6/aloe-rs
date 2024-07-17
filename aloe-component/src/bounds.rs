crate::ix!();

pub trait SetBounds {

    /**
      | Moves and resizes the window.
      | 
      | If the native window is contained in
      | another window, then the coordinates
      | are relative to the parent window's
      | origin, not the screen origin.
      | 
      | This should result in a callback to handleMovedOrResized().
      |
      */
    fn set_bounds(&mut self, 
            new_bounds:         &Rectangle<i32>,
            is_now_full_screen: bool);
}

pub trait GetFrameSize {

    /**
      | Returns the size of the window frame
      | that's around this window.
      | 
      | Whether or not the window has a normal
      | window frame depends on the flags that
      | were set when the window was created
      | by Component::addToDesktop()
      |
      */
    fn get_frame_size(&self) -> BorderSize<i32>;
}

pub trait HandleScreenSizeChange {

    /**
      | This is called if the screen resolution
      | changes.
      | 
      | A peer implementation must call this
      | if the monitor arrangement changes
      | or the available screen size changes.
      |
      */
    fn handle_screen_size_change(&mut self);
}

pub trait GetBounds {

    /**
      | Returns the current position and size
      | of the window.
      | 
      | If the native window is contained in
      | another window, then the coordinates
      | are relative to the parent window's
      | origin, not the screen origin.
      |
      */
    fn get_bounds(&self) -> Rectangle<i32>;
}

pub trait LocalToGlobalPoint {

    /**
      | Converts a position relative to the
      | top-left of this component to screen
      | coordinates.
      |
      */
    fn local_to_global(&mut self, relative_position: Point<f32>) -> Point<f32>;
}

pub trait GlobalToLocalPoint {

    /**
      | Converts a screen coordinate to a position
      | relative to the top-left of this component.
      |
      */
    fn global_to_local(&mut self, screen_position: Point<f32>) -> Point<f32>;
}

pub trait LocalToGlobalRectangle {

    /**
      | Converts a rectangle relative to the
      | top-left of this component to screen
      | coordinates.
      |
      */
    fn local_to_global(&mut self, relative_position: &Rectangle<i32>) -> Rectangle<i32>;
}

pub trait GlobalToLocalRectangle {

    /**
      | Converts a screen area to a position
      | relative to the top-left of this component.
      |
      */
    fn global_to_local(&mut self, screen_position: &Rectangle<i32>) -> Rectangle<i32>;
}
