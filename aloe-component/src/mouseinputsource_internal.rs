crate::ix!();

///---------------------------
#[no_copy]
#[leak_detector]
pub struct MouseInputSourceInternal<'a> {
    base:                              AsyncUpdater<'a>,

    index:                             i32,
    input_type:                        MouseInputSourceType,

    /**
      | NB: these are unscaled coords
      |
      */
    last_screen_pos:                   Point<f32>,

    /**
      | NB: these are unscaled coords
      |
      */
    unbounded_mouse_offset:            Point<f32>,

    button_state:                      ModifierKeys,
    pressure:                          f32,  // default = 0
    orientation:                       f32,  // default = 0
    rotation:                          f32,  // default = 0
    tiltx:                             f32,  // default = 0
    tilty:                             f32,  // default = 0
    is_unbounded_mouse_mode_on:        bool, // default = false
    is_cursor_visible_until_offscreen: bool, // default = false
    component_under_mouse:             WeakReference<Component<'a>>,
    last_non_inertial_wheel_target:    WeakReference<Component<'a>>,
    last_peer:                         *mut ComponentPeer<'a>, // default = nullptr
    current_cursor_handle:             *mut c_void,          // default = nullptr
    mouse_event_counter:               i32,                // default = 0
    mouse_downs:                       [RecentMouseDown; 4],
    last_time:                         Time,
    moved_significantly:               bool, // default = false
}

#[cfg(ALOE_DUMP_MOUSE_EVENTS)]
macro_rules! aloe_mouse_event_dbg {
    ($desc:ident) => {
        /*
                DBG ("Mouse " << desc << " #" << index 
                                                        << ": " << screenPosToLocalPos (comp, screenPos).toString() 
                                                        << " - Comp: " << String::toHexString ((pointer_sized_int) &comp));
        */
    }
}

#[cfg(not(ALOE_DUMP_MOUSE_EVENTS))]
macro_rules! aloe_mouse_event_dbg {
    ($desc:ident) => {
    }
}

impl<'a> MouseInputSourceInternal<'a> {

    pub fn new(
        i:  i32,
        ty: MouseInputSourceType

    ) -> Self {
    
        todo!();
        /*
        : index(i),
        : input_type(type),

        
        */
    }
    
    pub fn is_dragging(&self) -> bool {
        
        todo!();
        /*
            return buttonState.isAnyMouseButtonDown();
        */
    }
    
    pub fn get_component_under_mouse(&self) -> *mut Component {
        
        todo!();
        /*
            return componentUnderMouse.get();
        */
    }
    
    pub fn get_current_modifiers(&self) -> ModifierKeys {
        
        todo!();
        /*
            return ModifierKeys::currentModifiers.withoutMouseButtons().withFlags (buttonState.getRawFlags());
        */
    }
    
    pub fn get_peer(&mut self) -> *mut ComponentPeer {
        
        todo!();
        /*
            if (! ComponentPeer::isValidPeer (lastPeer))
                lastPeer = nullptr;

            return lastPeer;
        */
    }
    
    pub fn screen_pos_to_local_pos(
        comp: &mut Component,
        pos:  Point<f32>) -> Point<f32> {
        
        todo!();
        /*
            if (auto* peer = comp.getPeer())
            {
                pos = peer->globalToLocal (pos);
                auto& peerComp = peer->getComponent();
                return comp.getLocalPoint (&peerComp, ScalingHelpers::unscaledScreenPosToScaled (peerComp, pos));
            }

            return comp.getLocalPoint (nullptr, ScalingHelpers::unscaledScreenPosToScaled (comp, pos));
        */
    }
    
    pub fn find_component_at(&mut self, screen_pos: Point<f32>) -> *mut Component {
        
        todo!();
        /*
            if (auto* peer = getPeer())
            {
                auto relativePos = ScalingHelpers::unscaledScreenPosToScaled (peer->getComponent(),
                                                                              peer->globalToLocal (screenPos));
                auto& comp = peer->getComponent();

                // (the contains() call is needed to test for overlapping desktop windows)
                if (comp.containsInternal (relativePos))
                    return comp.getComponentAtInternal (relativePos);
            }

            return nullptr;
        */
    }
    
    pub fn get_screen_position(&self) -> Point<f32> {
        
        todo!();
        /*
            // This needs to return the live position if possible, but it mustn't update the lastScreenPos
            // value, because that can cause continuity problems.
            return ScalingHelpers::unscaledScreenPosToScaled (getRawScreenPosition());
        */
    }
    
    pub fn get_raw_screen_position(&self) -> Point<f32> {
        
        todo!();
        /*
            return unboundedMouseOffset + (inputType != MouseInputSource::MouseInputSourceType::touch ? MouseInputSource::getCurrentRawMousePosition()
                                                                                                 : lastScreenPos);
        */
    }
    
    pub fn set_screen_position(&mut self, p: Point<f32>)  {
        
        todo!();
        /*
            MouseInputSource::setRawMousePosition (ScalingHelpers::scaledScreenPosToUnscaled (p));
        */
    }
    
    pub fn is_pressure_valid(&self) -> bool {
        
        todo!();
        /*
            return pressure >= 0.0f && pressure <= 1.0f;
        */
    }
    
    pub fn is_orientation_valid(&self) -> bool {
        
        todo!();
        /*
            return orientation >= 0.0f && orientation <= MathConstants<float>::twoPi;
        */
    }
    
    pub fn is_rotation_valid(&self) -> bool {
        
        todo!();
        /*
            return rotation >= 0.0f && rotation <= MathConstants<float>::twoPi;
        */
    }
    
    pub fn is_tilt_valid(&self, isx: bool) -> bool {
        
        todo!();
        /*
            return isX ? (tiltX >= -1.0f && tiltX <= 1.0f) : (tiltY >= -1.0f && tiltY <= 1.0f);
        */
    }
    
    pub fn send_mouse_enter(&mut self, 
        comp:       &mut Component,
        screen_pos: Point<f32>,
        time:       Time)  {
        
        todo!();
        /*
            ALOE_MOUSE_EVENT_DBG ("enter")
            comp.internalMouseEnter (MouseInputSource (this), screenPosToLocalPos (comp, screenPos), time);
        */
    }
    
    pub fn send_mouse_exit(&mut self, 
        comp:       &mut Component,
        screen_pos: Point<f32>,
        time:       Time)  {
        
        todo!();
        /*
            ALOE_MOUSE_EVENT_DBG ("exit")
            comp.internalMouseExit (MouseInputSource (this), screenPosToLocalPos (comp, screenPos), time);
        */
    }
    
    pub fn send_mouse_move(&mut self, 
        comp:       &mut Component,
        screen_pos: Point<f32>,
        time:       Time)  {
        
        todo!();
        /*
            ALOE_MOUSE_EVENT_DBG ("move")
            comp.internalMouseMove (MouseInputSource (this), screenPosToLocalPos (comp, screenPos), time);
        */
    }
    
    pub fn send_mouse_down(&mut self, 
        comp:       &mut Component,
        screen_pos: Point<f32>,
        time:       Time)  {
        
        todo!();
        /*
            ALOE_MOUSE_EVENT_DBG ("down")
            comp.internalMouseDown (MouseInputSource (this), screenPosToLocalPos (comp, screenPos), time, pressure, orientation, rotation, tiltX, tiltY);
        */
    }
    
    pub fn send_mouse_drag(&mut self, 
        comp:       &mut Component,
        screen_pos: Point<f32>,
        time:       Time)  {
        
        todo!();
        /*
            ALOE_MOUSE_EVENT_DBG ("drag")
            comp.internalMouseDrag (MouseInputSource (this), screenPosToLocalPos (comp, screenPos), time, pressure, orientation, rotation, tiltX, tiltY);
        */
    }
    
    pub fn send_mouse_up(&mut self, 
        comp:       &mut Component,
        screen_pos: Point<f32>,
        time:       Time,
        old_mods:   ModifierKeys)  {
        
        todo!();
        /*
            ALOE_MOUSE_EVENT_DBG ("up")
                comp.internalMouseUp (MouseInputSource (this), screenPosToLocalPos (comp, screenPos), time, oldMods, pressure, orientation, rotation, tiltX, tiltY);
        */
    }
    
    pub fn send_mouse_wheel(&mut self, 
        comp:       &mut Component,
        screen_pos: Point<f32>,
        time:       Time,
        wheel:      &MouseWheelDetails)  {
        
        todo!();
        /*
            ALOE_MOUSE_EVENT_DBG ("wheel")
            comp.internalMouseWheel (MouseInputSource (this), screenPosToLocalPos (comp, screenPos), time, wheel);
        */
    }
    
    pub fn send_magnify_gesture(&mut self, 
        comp:       &mut Component,
        screen_pos: Point<f32>,
        time:       Time,
        amount:     f32)  {
        
        todo!();
        /*
            ALOE_MOUSE_EVENT_DBG ("magnify")
            comp.internalMagnifyGesture (MouseInputSource (this), screenPosToLocalPos (comp, screenPos), time, amount);
        */
    }

    /**
       (returns true if the button change caused
       a modal event loop)
      */
    pub fn set_buttons(&mut self, 
        screen_pos:       Point<f32>,
        time:             Time,
        new_button_state: ModifierKeys) -> bool {
        
        todo!();
        /*
            if (buttonState == newButtonState)
                return false;

            // (avoid sending a spurious mouse-drag when we receive a mouse-up)
            if (! (isDragging() && ! newButtonState.isAnyMouseButtonDown()))
                setScreenPos (screenPos, time, false);

            // (ignore secondary clicks when there's already a button down)
            if (buttonState.isAnyMouseButtonDown() == newButtonState.isAnyMouseButtonDown())
            {
                buttonState = newButtonState;
                return false;
            }

            auto lastCounter = mouseEventCounter;

            if (buttonState.isAnyMouseButtonDown())
            {
                if (auto* current = getComponentUnderMouse())
                {
                    auto oldMods = getCurrentModifiers();
                    buttonState = newButtonState; // must change this before calling sendMouseUp, in case it runs a modal loop

                    sendMouseUp (*current, screenPos + unboundedMouseOffset, time, oldMods);

                    if (lastCounter != mouseEventCounter)
                        return true; // if a modal loop happened, then newButtonState is no longer valid.
                }

                enableUnboundedMouseMovement (false, false);
            }

            buttonState = newButtonState;

            if (buttonState.isAnyMouseButtonDown())
            {
                Desktop::getInstance().incrementMouseClickCounter();

                if (auto* current = getComponentUnderMouse())
                {
                    registerMouseDown (screenPos, time, *current, buttonState,
                                       inputType == MouseInputSource::MouseInputSourceType::touch);
                    sendMouseDown (*current, screenPos, time);
                }
            }

            return lastCounter != mouseEventCounter;
        */
    }
    
    pub fn set_component_under_mouse(&mut self, 
        new_component: *mut Component,
        screen_pos:    Point<f32>,
        time:          Time)  {
        
        todo!();
        /*
            auto* current = getComponentUnderMouse();

            if (newComponent != current)
            {
                WeakReference<Component> safeNewComp (newComponent);
                auto originalButtonState = buttonState;

                if (current != nullptr)
                {
                    WeakReference<Component> safeOldComp (current);
                    setButtons (screenPos, time, ModifierKeys());

                    if (auto oldComp = safeOldComp.get())
                    {
                        componentUnderMouse = safeNewComp;
                        sendMouseExit (*oldComp, screenPos, time);
                    }

                    buttonState = originalButtonState;
                }

                componentUnderMouse = safeNewComp.get();
                current = safeNewComp.get();

                if (current != nullptr)
                    sendMouseEnter (*current, screenPos, time);

                revealCursor (false);
                setButtons (screenPos, time, originalButtonState);
            }
        */
    }
    
    pub fn set_peer(&mut self, 
        new_peer:   &mut ComponentPeer,
        screen_pos: Point<f32>,
        time:       Time)  {
        
        todo!();
        /*
            if (&newPeer != lastPeer)
            {
                setComponentUnderMouse (nullptr, screenPos, time);
                lastPeer = &newPeer;
                setComponentUnderMouse (findComponentAt (screenPos), screenPos, time);
            }
        */
    }
    
    pub fn set_screen_pos(&mut self, 
        new_screen_pos: Point<f32>,
        time:           Time,
        force_update:   bool)  {
        
        todo!();
        /*
            if (! isDragging())
                setComponentUnderMouse (findComponentAt (newScreenPos), newScreenPos, time);

            if (newScreenPos != lastScreenPos || forceUpdate)
            {
                cancelPendingUpdate();

                if (newScreenPos != MouseInputSource::offscreenMousePos)
                    lastScreenPos = newScreenPos;

                if (auto* current = getComponentUnderMouse())
                {
                    if (isDragging())
                    {
                        registerMouseDrag (newScreenPos);
                        sendMouseDrag (*current, newScreenPos + unboundedMouseOffset, time);

                        if (isUnboundedMouseModeOn)
                            handleUnboundedDrag (*current);
                    }
                    else
                    {
                        sendMouseMove (*current, newScreenPos, time);
                    }
                }

                revealCursor (false);
            }
        */
    }
    
    pub fn handle_event(&mut self, 
        new_peer:             &mut ComponentPeer,
        position_within_peer: Point<f32>,
        time:                 Time,
        new_mods:             ModifierKeys,
        new_pressure:         f32,
        new_orientation:      f32,
        pen:                  PenDetails)  {
        
        todo!();
        /*
            lastTime = time;

            const bool pressureChanged = (pressure != newPressure);
            pressure = newPressure;

            const bool orientationChanged = (orientation != newOrientation);
            orientation = newOrientation;

            const bool rotationChanged = (rotation != pen.rotation);
            rotation = pen.rotation;

            const bool tiltChanged = (tiltX != pen.tiltX || tiltY != pen.tiltY);
            tiltX = pen.tiltX;
            tiltY = pen.tiltY;

            const bool shouldUpdate = (pressureChanged || orientationChanged || rotationChanged || tiltChanged);

            ++mouseEventCounter;

            auto screenPos = newPeer.localToGlobal (positionWithinPeer);

            if (isDragging() && newMods.isAnyMouseButtonDown())
            {
                setScreenPos (screenPos, time, shouldUpdate);
            }
            else
            {
                setPeer (newPeer, screenPos, time);

                if (auto* peer = getPeer())
                {
                    if (setButtons (screenPos, time, newMods))
                        return; // some modal events have been dispatched, so the current event is now out-of-date

                    peer = getPeer();

                    if (peer != nullptr)
                        setScreenPos (screenPos, time, shouldUpdate);
                }
            }
        */
    }
    
    pub fn get_target_for_gesture(&mut self, 
        peer:                 &mut ComponentPeer,
        position_within_peer: Point<f32>,
        time:                 Time,
        screen_pos:           &mut Point<f32>) -> *mut Component {
        
        todo!();
        /*
            lastTime = time;
            ++mouseEventCounter;

            screenPos = peer.localToGlobal (positionWithinPeer);
            setPeer (peer, screenPos, time);
            setScreenPos (screenPos, time, false);
            triggerFakeMove();

            return getComponentUnderMouse();
        */
    }
    
    pub fn handle_wheel(&mut self, 
        peer:                 &mut ComponentPeer,
        position_within_peer: Point<f32>,
        time:                 Time,
        wheel:                &MouseWheelDetails)  {
        
        todo!();
        /*
            Desktop::getInstance().incrementMouseWheelCounter();
            Point<float> screenPos;

            // This will make sure that when the wheel spins in its inertial phase, any events
            // continue to be sent to the last component that the mouse was over when it was being
            // actively controlled by the user. This avoids confusion when scrolling through nested
            // scrollable components.
            if (lastNonInertialWheelTarget == nullptr || ! wheel.isInertial)
                lastNonInertialWheelTarget = getTargetForGesture (peer, positionWithinPeer, time, screenPos);
            else
                screenPos = peer.localToGlobal (positionWithinPeer);

            if (auto target = lastNonInertialWheelTarget.get())
                sendMouseWheel (*target, screenPos, time, wheel);
        */
    }
    
    pub fn handle_magnify_gesture(&mut self, 
        peer:                 &mut ComponentPeer,
        position_within_peer: Point<f32>,
        time:                 Time,
        scale_factor:         f32)  {
        
        todo!();
        /*
            Point<float> screenPos;

            if (auto* current = getTargetForGesture (peer, positionWithinPeer, time, screenPos))
                sendMagnifyGesture (*current, screenPos, time, scaleFactor);
        */
    }
    
    pub fn get_last_mouse_down_time(&self) -> Time {
        
        todo!();
        /*
            return mouseDowns[0].time;
        */
    }
    
    pub fn get_last_mouse_down_position(&self) -> Point<f32> {
        
        todo!();
        /*
            return ScalingHelpers::unscaledScreenPosToScaled (mouseDowns[0].position);
        */
    }
    
    pub fn get_number_of_multiple_clicks(&self) -> i32 {
        
        todo!();
        /*
            int numClicks = 1;

            if (! isLongPressOrDrag())
            {
                for (int i = 1; i < numElementsInArray (mouseDowns); ++i)
                {
                    if (mouseDowns[0].canBePartOfMultipleClickWith (mouseDowns[i], MouseEvent::getDoubleClickTimeout() * jmin (i, 2)))
                        ++numClicks;
                    else
                        break;
                }
            }

            return numClicks;
        */
    }
    
    pub fn is_long_press_or_drag(&self) -> bool {
        
        todo!();
        /*
            return movedSignificantly || lastTime > mouseDowns[0].time + RelativeTime::milliseconds (300);
        */
    }
    
    pub fn has_moved_significantly_since_pressed(&self) -> bool {
        
        todo!();
        /*
            return movedSignificantly;
        */
    }

    /**
       Deprecated method
      */
    pub fn has_mouse_moved_significantly_since_pressed(&self) -> bool {
        
        todo!();
        /*
            return isLongPressOrDrag();
        */
    }
    
    pub fn trigger_fake_move(&mut self)  {
        
        todo!();
        /*
            triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            setScreenPos (lastScreenPos, jmax (lastTime, Time::getCurrentTime()), true);
        */
    }
    
    pub fn enable_unbounded_mouse_movement(&mut self, 
        enable:                              bool,
        keep_cursor_visible_until_offscreen: bool)  {
        
        todo!();
        /*
            enable = enable && isDragging();
            isCursorVisibleUntilOffscreen = keepCursorVisibleUntilOffscreen;

            if (enable != isUnboundedMouseModeOn)
            {
                if ((! enable) && ((! isCursorVisibleUntilOffscreen) || ! unboundedMouseOffset.isOrigin()))
                {
                    // when released, return the mouse to within the component's bounds
                    if (auto* current = getComponentUnderMouse())
                        setScreenPosition (current->getScreenBounds().toFloat()
                                              .getConstrainedPoint (ScalingHelpers::unscaledScreenPosToScaled (lastScreenPos)));
                }

                isUnboundedMouseModeOn = enable;
                unboundedMouseOffset = {};

                revealCursor (true);
            }
        */
    }
    
    pub fn handle_unbounded_drag(&mut self, current: &mut Component)  {
        
        todo!();
        /*
            auto componentScreenBounds = ScalingHelpers::scaledScreenPosToUnscaled (current.getParentMonitorArea().reduced (2, 2).toFloat());

            if (! componentScreenBounds.contains (lastScreenPos))
            {
                auto componentCentre = current.getScreenBounds().toFloat().getCentre();
                unboundedMouseOffset += (lastScreenPos - ScalingHelpers::scaledScreenPosToUnscaled (componentCentre));
                setScreenPosition (componentCentre);
            }
            else if (isCursorVisibleUntilOffscreen
                      && (! unboundedMouseOffset.isOrigin())
                      && componentScreenBounds.contains (lastScreenPos + unboundedMouseOffset))
            {
                MouseInputSource::setRawMousePosition (lastScreenPos + unboundedMouseOffset);
                unboundedMouseOffset = {};
            }
        */
    }
    
    pub fn show_mouse_cursor(&mut self, 
        cursor:        MouseCursor,
        forced_update: bool)  {
        
        todo!();
        /*
            if (isUnboundedMouseModeOn && ((! unboundedMouseOffset.isOrigin()) || ! isCursorVisibleUntilOffscreen))
            {
                cursor = MouseCursor::NoCursor;
                forcedUpdate = true;
            }

            if (forcedUpdate || cursor.getHandle() != currentCursorHandle)
            {
                currentCursorHandle = cursor.getHandle();
                cursor.showInWindow (getPeer());
            }
        */
    }
    
    pub fn hide_cursor(&mut self)  {
        
        todo!();
        /*
            showMouseCursor (MouseCursor::NoCursor, true);
        */
    }
    
    pub fn reveal_cursor(&mut self, forced_update: bool)  {
        
        todo!();
        /*
            MouseCursor mc (MouseCursor::NormalCursor);

            if (auto* current = getComponentUnderMouse())
                mc = current->getLookAndFeel().getMouseCursorFor (*current);

            showMouseCursor (mc, forcedUpdate);
        */
    }
    
    pub fn register_mouse_down(&mut self, 
        screen_pos:      Point<f32>,
        time:            Time,
        component:       &mut Component,
        modifiers:       ModifierKeys,
        is_touch_source: bool)  {
        
        todo!();
        /*
            for (int i = numElementsInArray (mouseDowns); --i > 0;)
                mouseDowns[i] = mouseDowns[i - 1];

            mouseDowns[0].position = screenPos;
            mouseDowns[0].time = time;
            mouseDowns[0].buttons = modifiers.withOnlyMouseButtons();
            mouseDowns[0].isTouch = isTouchSource;

            if (auto* peer = component.getPeer())
                mouseDowns[0].peerID = peer->getUniqueID();
            else
                mouseDowns[0].peerID = 0;

            movedSignificantly = false;
            lastNonInertialWheelTarget = nullptr;
        */
    }
    
    pub fn register_mouse_drag(&mut self, screen_pos: Point<f32>)  {
        
        todo!();
        /*
            movedSignificantly = movedSignificantly || mouseDowns[0].position.getDistanceFrom (screenPos) >= 4;
        */
    }
}
