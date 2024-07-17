crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ScrollBar.h]

/**
  | A scrollbar component.
  | 
  | To use a scrollbar, set up its total range
  | using the setRangeLimits() method
  | - this sets the range of values it can
  | represent. Then you can use setCurrentRange()
  | to change the position and size of the
  | scrollbar's 'thumb'.
  | 
  | Registering a ScrollBar::ScrollBarListener
  | with the scrollbar will allow you to
  | find out when the user moves it, and you
  | can use the getCurrentRangeStart()
  | to find out where they moved it to.
  | 
  | The scrollbar will adjust its own visibility
  | according to whether its thumb size
  | allows it to actually be scrolled.
  | 
  | For most purposes, it's probably easier
  | to use a Viewport or ListBox instead
  | of handling a scrollbar directly.
  | 
  | @see ScrollBar::ScrollBarListener
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ScrollBar<'a> {
    base:                       Component<'a>,
    base2:                      AsyncUpdater<'a>,
    base3:                      Timer,
    total_range:                Range<f64>, //{ 0.0, 1.0 };
    visible_range:              Range<f64>, //{ 0.0, 1.0 };
    single_step_size:           f64, // default = 0.1
    drag_start_range:           f64, // default = 0
    thumb_area_start:           i32, // default = 0
    thumb_area_size:            i32, // default = 0
    thumb_start:                i32, // default = 0
    thumb_size:                 i32, // default = 0
    drag_start_mouse_pos:       i32, // default = 0
    last_mouse_pos:             i32, // default = 0
    initial_delay_in_millisecs: i32, // default = 100
    repeat_delay_in_millisecs:  i32, // default = 50
    minimum_delay_in_millisecs: i32, // default = 10
    vertical:                   bool,
    is_dragging_thumb:          bool, // default = false
    autohides:                  bool, // default = true
    user_visibility_flag:       bool, // default = false
    up_button:                  Box<ScrollbarButton<'a>>,
    down_button:                Box<ScrollbarButton<'a>>,
    listeners:                  ListenerList<Rc<RefCell<dyn ScrollBarListener>>>,
}

impl<'a> Drop for ScrollBar<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        upButton.reset();
        downButton.reset();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ScrollBar.cpp]

impl<'a> ScrollBar<'a> {

    /**
      | Returns true if the scrollbar is vertical,
      | false if it's horizontal.
      |
      */
    pub fn is_vertical(&self) -> bool {
        
        todo!();
        /*
            return vertical;
        */
    }

    /**
      | Returns the current limits on the thumb
      | position. @see setRangeLimits
      |
      */
    pub fn get_range_limit(&self) -> Range<f64> {
        
        todo!();
        /*
            return totalRange;
        */
    }

    /**
      | Returns the lower value that the thumb
      | can be set to.
      | 
      | This is the value set by setRangeLimits().
      |
      */
    pub fn get_minimum_range_limit(&self) -> f64 {
        
        todo!();
        /*
            return totalRange.getStart();
        */
    }

    /**
      | Returns the upper value that the thumb
      | can be set to.
      | 
      | This is the value set by setRangeLimits().
      |
      */
    pub fn get_maximum_range_limit(&self) -> f64 {
        
        todo!();
        /*
            return totalRange.getEnd();
        */
    }

    /**
      | Returns the current thumb range. @see
      | getCurrentRange, setCurrentRange
      |
      */
    pub fn get_current_range(&self) -> Range<f64> {
        
        todo!();
        /*
            return visibleRange;
        */
    }

    /**
      | Returns the position of the top of the
      | thumb. @see getCurrentRange, setCurrentRangeStart
      |
      */
    pub fn get_current_range_start(&self) -> f64 {
        
        todo!();
        /*
            return visibleRange.getStart();
        */
    }

    /**
      | Returns the current size of the thumb.
      | @see getCurrentRange, setCurrentRange
      |
      */
    pub fn get_current_range_size(&self) -> f64 {
        
        todo!();
        /*
            return visibleRange.getLength();
        */
    }

    /**
      | Returns the current step size. @see
      | setSingleStepSize
      |
      */
    pub fn get_single_step_size(&self) -> f64 {
        
        todo!();
        /*
            return singleStepSize;
        */
    }
    
    /**
      | Creates a Scrollbar.
      | 
      | -----------
      | @param isVertical
      | 
      | specifies whether the bar should be
      | a vertical or horizontal
      |
      */
    pub fn new(should_be_vertical: bool) -> Self {
    
        todo!();
        /*
        : vertical(shouldBeVertical),

            setRepaintsOnMouseActivity (true);
        setFocusContainerType (FocusContainerType::keyboardFocusContainer);
        */
    }
    
    /**
      | Sets the minimum and maximum values
      | that the bar will move between.
      | 
      | The bar's thumb will always be constrained
      | so that the entire thumb lies within
      | this range.
      | 
      | -----------
      | @param newRangeLimit
      | 
      | the new range.
      | ----------
      | @param notification
      | 
      | whether to send a notification of the
      | change to listeners.
      | 
      | A notification will only be sent if the
      | range has changed.
      | 
      | @see setCurrentRange
      |
      */
    pub fn set_range_limits_with_range(
        &mut self, 
        new_range_limit: Range<f64>,
        notification:    Option<NotificationType>

    )  {

        let notification: NotificationType 
            = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            if (totalRange != newRangeLimit)
        {
            totalRange = newRangeLimit;
            setCurrentRange (visibleRange, notification);
            updateThumbPosition();
        }
        */
    }
    
    /**
      | Sets the minimum and maximum values
      | that the bar will move between.
      | 
      | The bar's thumb will always be constrained
      | so that the entire thumb lies within
      | this range.
      | 
      | -----------
      | @param minimum
      | 
      | the new range minimum.
      | ----------
      | @param maximum
      | 
      | the new range maximum.
      | ----------
      | @param notification
      | 
      | whether to send a notification of the
      | change to listeners.
      | 
      | A notification will only be sent if the
      | range has changed.
      | 
      | @see setCurrentRange
      |
      */
    pub fn set_range_limits(
        &mut self, 
        new_minimum:  f64,
        new_maximum:  f64,
        notification: Option<NotificationType>

    ) {

        let notification: NotificationType 
            = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            jassert (newMaximum >= newMinimum); // these can't be the wrong way round!
        setRangeLimits (Range<double> (newMinimum, newMaximum), notification);
        */
    }
    
    /**
      | Changes the position of the scrollbar's
      | 'thumb'.
      | 
      | This sets both the position and size
      | of the thumb - to just set the position
      | without changing the size, you can use
      | setCurrentRangeStart().
      | 
      | If this method call actually changes
      | the scrollbar's position, it will trigger
      | an asynchronous call to ScrollBar::ScrollBarListener::scrollBarMoved()
      | for all the listeners that are registered.
      | 
      | The notification parameter can be used
      | to optionally send or inhibit a callback
      | to any scrollbar listeners.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the range was changed, or false
      | if nothing was changed. @see getCurrentRange.
      | setCurrentRangeStart
      |
      */
    pub fn set_current_range_with_range(
        &mut self, 
        new_range:    Range<f64>,
        notification: Option<NotificationType>

    ) -> bool {

        let notification: NotificationType 
            = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            auto constrainedRange = totalRange.constrainRange (newRange);

        if (visibleRange != constrainedRange)
        {
            visibleRange = constrainedRange;

            updateThumbPosition();

            if (notification != dontSendNotification)
                triggerAsyncUpdate();

            if (notification == sendNotificationSync)
                handleUpdateNowIfNeeded();

            return true;
        }

        return false;
        */
    }
    
    /**
      | Changes the position of the scrollbar's
      | 'thumb'.
      | 
      | This sets both the position and size
      | of the thumb - to just set the position
      | without changing the size, you can use
      | setCurrentRangeStart().
      | 
      | -----------
      | @param newStart
      | 
      | the top (or left) of the thumb, in the
      | range getMinimumRangeLimit() <= newStart
      | <= getMaximumRangeLimit(). If the
      | value is beyond these limits, it will
      | be clipped.
      | ----------
      | @param newSize
      | 
      | the size of the thumb, such that getMinimumRangeLimit()
      | <= newStart + newSize <= getMaximumRangeLimit().
      | If the size is beyond these limits, it
      | will be clipped.
      | ----------
      | @param notification
      | 
      | specifies if and how a callback should
      | be made to any listeners if the range
      | actually changes @see setCurrentRangeStart,
      | getCurrentRangeStart, getCurrentRangeSize
      |
      */
    pub fn set_current_range(
        &mut self, 
        new_start:    f64,
        new_size:     f64,
        notification: Option<NotificationType>

    )  {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            setCurrentRange (Range<double> (newStart, newStart + newSize), notification);
        */
    }
    
    /**
      | Moves the bar's thumb position.
      | 
      | This will move the thumb position without
      | changing the thumb size. Note that the
      | maximum thumb start position is (getMaximumRangeLimit()
      | - getCurrentRangeSize()).
      | 
      | If this method call actually changes
      | the scrollbar's position, it will trigger
      | an asynchronous call to ScrollBar::ScrollBarListener::scrollBarMoved()
      | for all the listeners that are registered.
      | 
      | @see setCurrentRange
      |
      */
    pub fn set_current_range_start(
        &mut self, 
        new_start:    f64,
        notification: Option<NotificationType>
    )  {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            setCurrentRange (visibleRange.movedToStartAt (newStart), notification);
        */
    }
    
    /**
      | Sets the amount by which the up and down
      | buttons will move the bar.
      | 
      | The value here is in terms of the total
      | range, and is added or subtracted from
      | the thumb position when the user clicks
      | an up/down (or left/right) button.
      |
      */
    pub fn set_single_step_size(&mut self, new_single_step_size: f64)  {
        
        todo!();
        /*
            singleStepSize = newSingleStepSize;
        */
    }
    
    /**
      | Moves the scrollbar by a number of single-steps.
      | 
      | This will move the bar by a multiple of
      | its single-step interval (as specified
      | using the setSingleStepSize() method).
      | 
      | A positive value here will move the bar
      | down or to the right, a negative value
      | moves it up or to the left.
      | 
      | -----------
      | @param howManySteps
      | 
      | the number of steps to move the scrollbar
      | ----------
      | @param notification
      | 
      | whether to send a notification of the
      | change to listeners.
      | 
      | A notification will only be sent if the
      | position has changed.
      | 
      | -----------
      | @return
      | 
      | true if the scrollbar's position actually
      | changed.
      |
      */
    pub fn move_scrollbar_in_steps(
        &mut self, 
        how_many_steps: i32,
        notification:   Option<NotificationType>

    ) -> bool {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            return setCurrentRange (visibleRange + howManySteps * singleStepSize, notification);
        */
    }
    
    /**
      | Moves the scroll bar up or down in pages.
      | 
      | This will move the bar by a multiple of
      | its current thumb size, effectively
      | doing a page-up or down.
      | 
      | A positive value here will move the bar
      | down or to the right, a negative value
      | moves it up or to the left.
      | 
      | -----------
      | @param howManyPages
      | 
      | the number of pages to move the scrollbar
      | ----------
      | @param notification
      | 
      | whether to send a notification of the
      | change to listeners.
      | 
      | A notification will only be sent if the
      | position has changed.
      | 
      | -----------
      | @return
      | 
      | true if the scrollbar's position actually
      | changed.
      |
      */
    pub fn move_scrollbar_in_pages(
        &mut self, 
        how_many_pages: i32,
        notification:   Option<NotificationType>

    ) -> bool {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            return setCurrentRange (visibleRange + howManyPages * visibleRange.getLength(), notification);
        */
    }
    
    /**
      | Scrolls to the top (or left).
      | 
      | This is the same as calling setCurrentRangeStart
      | (getMinimumRangeLimit());
      | 
      | -----------
      | @param notification
      | 
      | whether to send a notification of the
      | change to listeners.
      | 
      | A notification will only be sent if the
      | position has changed.
      | 
      | -----------
      | @return
      | 
      | true if the scrollbar's position actually
      | changed.
      |
      */
    pub fn scroll_to_top(
        &mut self, 
        notification: Option<NotificationType>

    ) -> bool {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            return setCurrentRange (visibleRange.movedToStartAt (getMinimumRangeLimit()), notification);
        */
    }
    
    /**
      | Scrolls to the bottom (or right).
      | 
      | This is the same as calling setCurrentRangeStart
      | (getMaximumRangeLimit() - getCurrentRangeSize());
      | 
      | -----------
      | @param notification
      | 
      | whether to send a notification of the
      | change to listeners.
      | 
      | A notification will only be sent if the
      | position has changed.
      | 
      | -----------
      | @return
      | 
      | true if the scrollbar's position actually
      | changed.
      |
      */
    pub fn scroll_to_bottom(
        &mut self, 
        notification: Option<NotificationType>

    ) -> bool {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            return setCurrentRange (visibleRange.movedToEndAt (getMaximumRangeLimit()), notification);
        */
    }
    
    /**
      | Changes the delay before the up and down
      | buttons autorepeat when they are held
      | down.
      | 
      | For an explanation of what the parameters
      | are for, see Button::setRepeatSpeed().
      | 
      | @see Button::setRepeatSpeed
      |
      */
    pub fn set_button_repeat_speed(
        &mut self, 
        new_initial_delay: i32,
        new_repeat_delay:  i32,
        new_minimum_delay: Option<i32>

    ) {

        let new_minimum_delay: i32 = new_minimum_delay.unwrap_or(-1);
        
        todo!();
        /*
            initialDelayInMillisecs = newInitialDelay;
        repeatDelayInMillisecs  = newRepeatDelay;
        minimumDelayInMillisecs = newMinimumDelay;

        if (upButton != nullptr)
        {
            upButton  ->setRepeatSpeed (newInitialDelay, newRepeatDelay, newMinimumDelay);
            downButton->setRepeatSpeed (newInitialDelay, newRepeatDelay, newMinimumDelay);
        }
        */
    }
    
    /**
      | Registers a listener that will be called
      | when the scrollbar is moved.
      |
      */
    pub fn add_listener(&mut self, listener: *mut dyn ScrollBarListener)  {
        
        todo!();
        /*
            listeners.add (listener);
        */
    }
    
    /**
      | Deregisters a previously-registered
      | listener.
      |
      */
    pub fn remove_listener(&mut self, listener: *mut dyn ScrollBarListener)  {
        
        todo!();
        /*
            listeners.remove (listener);
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            auto start = visibleRange.getStart(); // (need to use a temp variable for VC7 compatibility)
        listeners.call ([this, start] (ScrollBarListener& l) { l.scrollBarMoved (this, start); });
        */
    }
    
    pub fn update_thumb_position(&mut self)  {
        
        todo!();
        /*
            auto minimumScrollBarThumbSize = getLookAndFeel().getMinimumScrollbarThumbSize (*this);

        int newThumbSize = roundToInt (totalRange.getLength() > 0 ? (visibleRange.getLength() * thumbAreaSize) / totalRange.getLength()
                                                                  : thumbAreaSize);

        if (newThumbSize < minimumScrollBarThumbSize)
            newThumbSize = jmin (minimumScrollBarThumbSize, thumbAreaSize - 1);

        if (newThumbSize > thumbAreaSize)
            newThumbSize = thumbAreaSize;

        int newThumbStart = thumbAreaStart;

        if (totalRange.getLength() > visibleRange.getLength())
            newThumbStart += roundToInt (((visibleRange.getStart() - totalRange.getStart()) * (thumbAreaSize - newThumbSize))
                                             / (totalRange.getLength() - visibleRange.getLength()));

        Component::setVisible (getVisibility());

        if (thumbStart != newThumbStart  || thumbSize != newThumbSize)
        {
            auto repaintStart = jmin (thumbStart, newThumbStart) - 4;
            auto repaintSize = jmax (thumbStart + thumbSize, newThumbStart + newThumbSize) + 8 - repaintStart;

            if (vertical)
                repaint (0, repaintStart, getWidth(), repaintSize);
            else
                repaint (repaintStart, 0, repaintSize, getHeight());

            thumbStart = newThumbStart;
            thumbSize = newThumbSize;
        }
        */
    }
    
    /**
      | Changes the scrollbar's direction.
      | 
      | You'll also need to resize the bar appropriately
      | - this just changes its internal layout.
      | 
      | -----------
      | @param shouldBeVertical
      | 
      | true makes it vertical; false makes
      | it horizontal.
      |
      */
    pub fn set_orientation(&mut self, should_be_vertical: bool)  {
        
        todo!();
        /*
            if (vertical != shouldBeVertical)
        {
            vertical = shouldBeVertical;

            if (upButton != nullptr)
            {
                upButton->direction    = vertical ? 0 : 3;
                downButton->direction  = vertical ? 2 : 1;
            }

            updateThumbPosition();
        }
        */
    }
    
    /**
      | Tells the scrollbar whether to make
      | itself invisible when not needed.
      | 
      | The default behaviour is for a scrollbar
      | to become invisible when the thumb fills
      | the whole of its range (i.e. when it can't
      | be moved). Setting this value to false
      | forces the bar to always be visible.
      | @see autoHides()
      |
      */
    pub fn set_auto_hide(&mut self, should_hide_when_full_range: bool)  {
        
        todo!();
        /*
            autohides = shouldHideWhenFullRange;
        updateThumbPosition();
        */
    }
    
    /**
      | Returns true if this scrollbar is set
      | to auto-hide when its thumb is as big
      | as its maximum range. @see setAutoHide
      |
      */
    pub fn auto_hides(&self) -> bool {
        
        todo!();
        /*
            return autohides;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (thumbAreaSize > 0)
        {
            auto& lf = getLookAndFeel();

            auto thumb = (thumbAreaSize > lf.getMinimumScrollbarThumbSize (*this))
                           ? thumbSize : 0;

            if (vertical)
                lf.drawScrollbar (g, *this, 0, thumbAreaStart, getWidth(), thumbAreaSize,
                                  vertical, thumbStart, thumb, isMouseOver(), isMouseButtonDown());
            else
                lf.drawScrollbar (g, *this, thumbAreaStart, 0, thumbAreaSize, getHeight(),
                                  vertical, thumbStart, thumb, isMouseOver(), isMouseButtonDown());
        }
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            setComponentEffect (getLookAndFeel().getScrollbarEffect());

        if (isVisible())
            resized();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto length = vertical ? getHeight() : getWidth();

        auto& lf = getLookAndFeel();
        bool buttonsVisible = lf.areScrollbarButtonsVisible();
        int buttonSize = 0;

        if (buttonsVisible)
        {
            if (upButton == nullptr)
            {
                upButton  .reset (new ScrollbarButton (vertical ? 0 : 3, *this));
                downButton.reset (new ScrollbarButton (vertical ? 2 : 1, *this));

                addAndMakeVisible (upButton.get());
                addAndMakeVisible (downButton.get());

                setButtonRepeatSpeed (initialDelayInMillisecs, repeatDelayInMillisecs, minimumDelayInMillisecs);
            }

            buttonSize = jmin (lf.getScrollbarButtonSize (*this), length / 2);
        }
        else
        {
            upButton.reset();
            downButton.reset();
        }

        if (length < 32 + lf.getMinimumScrollbarThumbSize (*this))
        {
            thumbAreaStart = length / 2;
            thumbAreaSize = 0;
        }
        else
        {
            thumbAreaStart = buttonSize;
            thumbAreaSize = length - 2 * buttonSize;
        }

        if (upButton != nullptr)
        {
            auto r = getLocalBounds();

            if (vertical)
            {
                upButton->setBounds (r.removeFromTop (buttonSize));
                downButton->setBounds (r.removeFromBottom (buttonSize));
            }
            else
            {
                upButton->setBounds (r.removeFromLeft (buttonSize));
                downButton->setBounds (r.removeFromRight (buttonSize));
            }
        }

        updateThumbPosition();
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            lookAndFeelChanged();
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            isDraggingThumb = false;
        lastMousePos = vertical ? e.y : e.x;
        dragStartMousePos = lastMousePos;
        dragStartRange = visibleRange.getStart();

        if (dragStartMousePos < thumbStart)
        {
            moveScrollbarInPages (-1);
            startTimer (400);
        }
        else if (dragStartMousePos >= thumbStart + thumbSize)
        {
            moveScrollbarInPages (1);
            startTimer (400);
        }
        else
        {
            isDraggingThumb = (thumbAreaSize > getLookAndFeel().getMinimumScrollbarThumbSize (*this))
                                && (thumbAreaSize > thumbSize);
        }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            auto mousePos = vertical ? e.y : e.x;

        if (isDraggingThumb && lastMousePos != mousePos && thumbAreaSize > thumbSize)
        {
            auto deltaPixels = mousePos - dragStartMousePos;

            setCurrentRangeStart (dragStartRange
                                    + deltaPixels * (totalRange.getLength() - visibleRange.getLength())
                                        / (thumbAreaSize - thumbSize));
        }

        lastMousePos = mousePos;
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            isDraggingThumb = false;
        stopTimer();
        repaint();
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        _0:    &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            float increment = 10.0f * (vertical ? wheel.deltaY : wheel.deltaX);

        if (increment < 0)
            increment = jmin (increment, -1.0f);
        else if (increment > 0)
            increment = jmax (increment, 1.0f);

        setCurrentRange (visibleRange - singleStepSize * increment);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (isMouseButtonDown())
        {
            startTimer (40);

            if (lastMousePos < thumbStart)
                setCurrentRange (visibleRange - visibleRange.getLength());
            else if (lastMousePos > thumbStart + thumbSize)
                setCurrentRangeStart (visibleRange.getEnd());
        }
        else
        {
            stopTimer();
        }
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (isVisible())
        {
            if (key == KeyPress::upKey || key == KeyPress::leftKey)    return moveScrollbarInSteps (-1);
            if (key == KeyPress::downKey || key == KeyPress::rightKey) return moveScrollbarInSteps (1);
            if (key == KeyPress::pageUpKey)                            return moveScrollbarInPages (-1);
            if (key == KeyPress::pageDownKey)                          return moveScrollbarInPages (1);
            if (key == KeyPress::homeKey)                              return scrollToTop();
            if (key == KeyPress::endKey)                               return scrollToBottom();
        }

        return false;
        */
    }
    
    pub fn set_visible(&mut self, should_be_visible: bool)  {
        
        todo!();
        /*
            if (userVisibilityFlag != shouldBeVisible)
        {
            userVisibilityFlag = shouldBeVisible;
            Component::setVisible (getVisibility());
        }
        */
    }
    
    pub fn get_visibility(&self) -> bool {
        
        todo!();
        /*
            if (! userVisibilityFlag)
            return false;

        return (! autohides) || (totalRange.getLength() > visibleRange.getLength()
                                        && visibleRange.getLength() > 0.0);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            class ValueInterface  : public AccessibilityRangedNumericValueInterface
        {
        
            explicit ValueInterface (ScrollBar& scrollBarToWrap)  : scrollBar (scrollBarToWrap) {}

            bool isReadOnly() const override          { return false; }

            double getCurrentValue() const override   { return scrollBar.getCurrentRangeStart(); }
            void setValue (double newValue) override  { scrollBar.setCurrentRangeStart (newValue); }

            AccessibleValueRange getRange() const override
            {
                if (scrollBar.getRangeLimit().isEmpty())
                    return {};

                return { { scrollBar.getMinimumRangeLimit(), scrollBar.getMaximumRangeLimit() },
                         scrollBar.getSingleStepSize() };
            }

        
            ScrollBar& scrollBar;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (ValueInterface)
        };

        return std::make_unique<AccessibilityHandler> (*this,
                                                       AccessibilityRole::scrollBar,
                                                       AccessibilityActions{},
                                                       AccessibilityHandler::Interfaces { std::make_unique<ValueInterface> (*this) });
        */
    }
}
