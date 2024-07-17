crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_MouseInputSource.h]

/**
  | Possible mouse input sources.
  |
  */
pub enum MouseInputSourceType
{
    mouse,
    touch,
    pen
}

/**
  | A default value for pressure, which
  | is used when a device doesn't support
  | it, or for mouse-moves, mouse-ups,
  | etc.
  |
  */
pub const MOUSE_INPUT_SOURCE_INVALID_PRESSURE:    f32 = 0.0;

/**
  | A default value for orientation, which
  | is used when a device doesn't support
  | it
  |
  */
pub const MOUSE_INPUT_SOURCE_INVALID_ORIENTATION: f32 = 0.0;

/**
  | A default value for rotation, which
  | is used when a device doesn't support
  | it
  |
  */
pub const MOUSE_INPUT_SOURCE_INVALID_ROTATION:    f32 = 0.0;

/**
  | Default values for tilt, which are used
  | when a device doesn't support it
  |
  */
pub const MOUSE_INPUT_SOURCE_INVALID_TILTX:       f32 = 0.0;
pub const MOUSE_INPUT_SOURCE_INVALID_TILTY:       f32 = 0.0;

/**
  | An offscreen mouse position used when
  | triggering mouse exits where we don't
  | want to move the cursor over an existing
  | component.
  |
  */
pub const MOUSE_INPUT_SOURCE_OFFSCREEN_MOUSE_POS: Point<f32> = Point::new(-10.0, -10.0);

/**
  | Represents a linear source of mouse
  | events from a mouse device or individual
  | finger in a multi-touch environment.
  | 
  | Each MouseEvent object contains a reference
  | to the MouseInputSource that generated
  | it. In an environment with a single mouse
  | for input, all events will come from
  | the same source, but in a multi-touch
  | system, there may be multiple MouseInputSource
  | objects active, each representing
  | a stream of events coming from a particular
  | finger.
  | 
  | Events coming from a single MouseInputSource
  | are always sent in a fixed and predictable
  | order: a mouseMove will never be called
  | without a mouseEnter having been sent
  | beforehand, the only events that can
  | happen between a mouseDown and its corresponding
  | mouseUp are mouseDrags, etc.
  | 
  | When there are multiple touches arriving
  | from multiple MouseInputSources,
  | their event streams may arrive in an
  | interleaved order, so you should use
  | the getIndex() method to find out which
  | finger each event came from.
  | 
  | @see MouseEvent
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct MouseInputSource<'a> {

    pimpl: *mut MouseInputSourceInternal<'a>,
}

impl<'a> PartialEq<MouseInputSource<'a>> for MouseInputSource<'a> {
    
    #[inline] fn eq(&self, other: &MouseInputSource<'a>) -> bool {
        todo!();
        /*
            return pimpl == other.pimpl;
        */
    }
}

impl<'a> Eq for MouseInputSource<'a> {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_MouseInputSource.cpp]
impl<'a> MouseInputSource<'a> {
    
    #[cfg(target_os="linux")]
    pub fn get_current_raw_mouse_position(&mut self) -> Point<f32> {
        
        todo!();
        /*
            return Desktop::getInstance().getDisplays().physicalToLogical (XWindowSystem::getInstance()->getCurrentMousePosition());
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn set_raw_mouse_position(&mut self, new_position: Point<f32>)  {
        
        todo!();
        /*
            XWindowSystem::getInstance()->setMousePosition (Desktop::getInstance().getDisplays().logicalToPhysical (newPosition));
        */

    }

    pub fn new(s: *mut MouseInputSourceInternal) -> Self {
    
        todo!();
        /*
        : pimpl(s),
        */
    }
    
    pub fn new_with_mouse_input_source(other: &MouseInputSource) -> Self {
    
        todo!();
        /*
        : pimpl(other.pimpl),
        */
    }
    
    pub fn assign_from(&mut self, other: &MouseInputSource) -> &mut MouseInputSource {
        
        todo!();
        /*
            pimpl = other.pimpl;
        return *this;
        */
    }
    
    /**
      | Returns the type of input source that
      | this object represents.
      |
      */
    pub fn get_type(&self) -> MouseInputSourceType {
        
        todo!();
        /*
            return pimpl->inputType;
        */
    }
    
    /**
      | Returns true if this object represents
      | a normal desk-based mouse device.
      |
      */
    pub fn is_mouse(&self) -> bool {
        
        todo!();
        /*
            return (getType() == MouseInputSource::MouseInputSourceType::mouse);
        */
    }
    
    /**
      | Returns true if this object represents
      | a source of touch events.
      |
      */
    pub fn is_touch(&self) -> bool {
        
        todo!();
        /*
            return (getType() == MouseInputSource::MouseInputSourceType::touch);
        */
    }
    
    /**
      | Returns true if this object represents
      | a pen device.
      |
      */
    pub fn is_pen(&self) -> bool {
        
        todo!();
        /*
            return (getType() == MouseInputSource::MouseInputSourceType::pen);
        */
    }
    
    /**
      | Returns true if this source has an on-screen
      | pointer that can hover over items without
      | clicking them.
      |
      */
    pub fn can_hover(&self) -> bool {
        
        todo!();
        /*
            return ! isTouch();
        */
    }
    
    /**
      | Returns true if this source may have
      | a scroll wheel.
      |
      */
    pub fn has_mouse_wheel(&self) -> bool {
        
        todo!();
        /*
            return ! isTouch();
        */
    }
    
    /**
      | Returns this source's index in the global
      | list of possible sources.
      | 
      | If the system only has a single mouse,
      | there will only be a single MouseInputSource
      | with an index of 0.
      | 
      | If the system supports multi-touch
      | input, then the index will represent
      | a finger number, starting from 0. When
      | the first touch event begins, it will
      | have finger number 0, and then if a second
      | touch happens while the first is still
      | down, it will have index 1, etc.
      |
      */
    pub fn get_index(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->index;
        */
    }
    
    /**
      | Returns true if this device is currently
      | being pressed.
      |
      */
    pub fn is_dragging(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isDragging();
        */
    }
    
    /**
      | Returns the last-known screen position
      | of this source.
      |
      */
    pub fn get_screen_position(&self) -> Point<f32> {
        
        todo!();
        /*
            return pimpl->getScreenPosition();
        */
    }
    
    /**
      | Returns the last-known screen position
      | of this source without any scaling applied.
      |
      */
    pub fn get_raw_screen_position(&self) -> Point<f32> {
        
        todo!();
        /*
            return pimpl->getRawScreenPosition();
        */
    }
    
    /**
      | Returns a set of modifiers that indicate
      | which buttons are currently held down
      | on this device.
      |
      */
    pub fn get_current_modifiers(&self) -> ModifierKeys {
        
        todo!();
        /*
            return pimpl->getCurrentModifiers();
        */
    }
    
    /**
      | Returns the device's current touch
      | or pen pressure.
      | 
      | The range is 0 (soft) to 1 (hard).
      | 
      | If the input device doesn't provide
      | any pressure data, it may return a negative
      | value here, or 0.0 or 1.0, depending
      | on the platform.
      |
      */
    pub fn get_current_pressure(&self) -> f32 {
        
        todo!();
        /*
            return pimpl->pressure;
        */
    }
    
    /**
      | Returns true if the current pressure
      | value is meaningful.
      |
      */
    pub fn is_pressure_valid(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isPressureValid();
        */
    }
    
    /**
      | Returns the device's current orientation
      | in radians. 0 indicates a touch pointer
      | aligned with the x-axis and pointing
      | from left to right; increasing values
      | indicate rotation in the clockwise
      | direction. Only reported by a touch
      | pointer.
      |
      */
    pub fn get_current_orientation(&self) -> f32 {
        
        todo!();
        /*
            return pimpl->orientation;
        */
    }
    
    /**
      | Returns true if the current orientation
      | value is meaningful.
      |
      */
    pub fn is_orientation_valid(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isOrientationValid();
        */
    }
    
    /**
      | Returns the device's current rotation.
      | Indicates the clockwise rotation,
      | or twist, of the pointer in radians.
      | The default is 0. Only reported by a pen
      | pointer.
      |
      */
    pub fn get_current_rotation(&self) -> f32 {
        
        todo!();
        /*
            return pimpl->rotation;
        */
    }
    
    /**
      | Returns true if the current rotation
      | value is meaningful.
      |
      */
    pub fn is_rotation_valid(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isRotationValid();
        */
    }
    
    /**
      | Returns the angle of tilt of the pointer
      | in a range of -1.0 to 1.0 either in the
      | x- or y-axis. The default is 0.
      | 
      | If x-axis, a positive value indicates
      | a tilt to the right and if y-axis, a positive
      | value indicates a tilt toward the user.
      | 
      | Only reported by a pen pointer.
      |
      */
    pub fn get_current_tilt(&self, tiltx: bool) -> f32 {
        
        todo!();
        /*
            return tiltX ? pimpl->tiltX : pimpl->tiltY;
        */
    }
    
    /**
      | Returns true if the current tilt value
      | (either x- or y-axis) is meaningful.
      |
      */
    pub fn is_tilt_valid(&self, isx: bool) -> bool {
        
        todo!();
        /*
            return pimpl->isTiltValid (isX);
        */
    }
    
    /**
      | Returns the component that was last
      | known to be under this pointer.
      |
      */
    pub fn get_component_under_mouse(&self) -> *mut Component {
        
        todo!();
        /*
            return pimpl->getComponentUnderMouse();
        */
    }
    
    /**
      | Tells the device to dispatch a mouse-move
      | or mouse-drag event.
      | 
      | This is asynchronous - the event will
      | occur on the message thread.
      |
      */
    pub fn trigger_fake_move(&self)  {
        
        todo!();
        /*
            pimpl->triggerFakeMove();
        */
    }
    
    /**
      | Returns the number of clicks that should
      | be counted as belonging to the current
      | mouse event.
      | 
      | So the mouse is currently down and it's
      | the second click of a double-click,
      | this will return 2.
      |
      */
    pub fn get_number_of_multiple_clicks(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->getNumberOfMultipleClicks();
        */
    }
    
    /**
      | Returns the time at which the last mouse-down
      | occurred.
      |
      */
    pub fn get_last_mouse_down_time(&self) -> Time {
        
        todo!();
        /*
            return pimpl->getLastMouseDownTime();
        */
    }
    
    /**
      | Returns the screen position at which
      | the last mouse-down occurred.
      |
      */
    pub fn get_last_mouse_down_position(&self) -> Point<f32> {
        
        todo!();
        /*
            return pimpl->getLastMouseDownPosition();
        */
    }
    
    /**
      | Returns true if this input source represents
      | a long-press or drag interaction i.e.
      | it has been held down for a significant
      | amount of time or it has been dragged
      | more than a couple of pixels from the
      | place it was pressed.
      |
      */
    pub fn is_long_press_or_drag(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isLongPressOrDrag();
        */
    }
    
    /**
      | Returns true if this input source has
      | been dragged more than a couple of pixels
      | from the place it was pressed.
      |
      */
    pub fn has_moved_significantly_since_pressed(&self) -> bool {
        
        todo!();
        /*
            return pimpl->hasMovedSignificantlySincePressed();
        */
    }
    
    /**
      | Returns true if this mouse can be moved
      | indefinitely in any direction without
      | running out of space.
      |
      */
    pub fn can_do_unbounded_movement(&self) -> bool {
        
        todo!();
        /*
            return ! isTouch();
        */
    }
    
    /**
      | Allows the mouse to move beyond the edges
      | of the screen.
      | 
      | Calling this method when the mouse button
      | is currently pressed will remove the
      | cursor from the screen and allow the
      | mouse to (seem to) move beyond the edges
      | of the screen.
      | 
      | This means that the coordinates returned
      | to mouseDrag() will be unbounded, and
      | this can be used for things like custom
      | slider controls or dragging objects
      | around, where movement would be otherwise
      | be limited by the mouse hitting the edges
      | of the screen.
      | 
      | The unbounded mode is automatically
      | turned off when the mouse button is released,
      | or it can be turned off explicitly by
      | calling this method again.
      | 
      | -----------
      | @param isEnabled
      | 
      | whether to turn this mode on or off
      | ----------
      | @param keepCursorVisibleUntilOffscreen
      | 
      | if set to false, the cursor will immediately
      | be hidden; if true, it will only be hidden
      | when it is moved beyond the edge of the
      | screen
      |
      */
    pub fn enable_unbounded_mouse_movement(
        &self, 
        is_enabled:                          bool,
        keep_cursor_visible_until_offscreen: Option<bool>

    ) {

        let keep_cursor_visible_until_offscreen: bool = keep_cursor_visible_until_offscreen.unwrap_or(false);
        
        todo!();
        /*
            pimpl->enableUnboundedMouseMovement (isEnabled, keepCursorVisibleUntilOffscreen);
        */
    }
    
    /**
      | Returns true if this source is currently
      | in "unbounded" mode.
      |
      */
    pub fn is_unbounded_mouse_movement_enabled(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isUnboundedMouseModeOn;
        */
    }
    
    /**
      | Returns true if this input source uses
      | a visible mouse cursor.
      |
      */
    pub fn has_mouse_cursor(&self) -> bool {
        
        todo!();
        /*
            return ! isTouch();
        */
    }
    
    /**
      | Changes the mouse cursor, (if there
      | is one).
      |
      */
    pub fn show_mouse_cursor(&mut self, cursor: &MouseCursor)  {
        
        todo!();
        /*
            pimpl->showMouseCursor (cursor, false);
        */
    }
    
    /**
      | Hides the mouse cursor (if there is one).
      |
      */
    pub fn hide_cursor(&mut self)  {
        
        todo!();
        /*
            pimpl->hideCursor();
        */
    }
    
    /**
      | Un-hides the mouse cursor if it was hidden
      | by hideCursor().
      |
      */
    pub fn reveal_cursor(&mut self)  {
        
        todo!();
        /*
            pimpl->revealCursor (false);
        */
    }
    
    /**
      | Forces an update of the mouse cursor
      | for whatever component it's currently
      | over.
      |
      */
    pub fn force_mouse_cursor_update(&mut self)  {
        
        todo!();
        /*
            pimpl->revealCursor (true);
        */
    }
    
    /**
      | Attempts to set this mouse pointer's
      | screen position.
      |
      */
    pub fn set_screen_position(&mut self, p: Point<f32>)  {
        
        todo!();
        /*
            pimpl->setScreenPosition (p);
        */
    }
    
    pub fn handle_event(&mut self, 
        peer:        &mut ComponentPeer,
        pos:         Point<f32>,
        time:        i64,
        mods:        ModifierKeys,
        pressure:    f32,
        orientation: f32,
        pen_details: &PenDetails)  {
        
        todo!();
        /*
            pimpl->handleEvent (peer, pos, Time (time), mods.withOnlyMouseButtons(), pressure, orientation, penDetails);
        */
    }
    
    pub fn handle_wheel(&mut self, 
        peer:  &mut ComponentPeer,
        pos:   Point<f32>,
        time:  i64,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            pimpl->handleWheel (peer, pos, Time (time), wheel);
        */
    }
    
    pub fn handle_magnify_gesture(&mut self, 
        peer:         &mut ComponentPeer,
        pos:          Point<f32>,
        time:         i64,
        scale_factor: f32)  {
        
        todo!();
        /*
            pimpl->handleMagnifyGesture (peer, pos, Time (time), scaleFactor);
        */
    }
    
    #[deprecated]
    pub fn has_mouse_moved_significantly_since_pressed(&self) -> bool {
        
        todo!();
        /*
            return pimpl->hasMouseMovedSignificantlySincePressed();
        */
    }
}
