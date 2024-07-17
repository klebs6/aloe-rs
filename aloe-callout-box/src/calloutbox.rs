crate::ix!();

pub const call_out_box_dismiss_command_id: i32 = 0x4f83a04b;

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_CallOutBox.h]

/**
  | A box with a small arrow that can be used
  | as a temporary pop-up window to show
  | extra controls when a button or other
  | component is clicked.
  | 
  | Using one of these is similar to having
  | a popup menu attached to a button or other
  | component - but it looks fancier, and
  | has an arrow that can indicate the object
  | that it applies to.
  | 
  | The class works best when shown modally,
  | but obviously running modal loops is
  | evil and must never be done, so the launchAsynchronously
  | method is provided as a handy way of launching
  | an instance of a CallOutBox and automatically
  | managing its lifetime, e.g.
  | 
  | -----------
  | @code
  | 
  | void mouseUp (const MouseEvent&)
  | {
  |     auto content = std::make_unique<FoobarContentComp>();
  |     content->setSize (300, 300);
  | 
  |     auto& myBox = CallOutBox::launchAsynchronously (std::move (content),
  |                                                     getScreenBounds(),
  |                                                     nullptr);
  | }
  | 
  | The call-out will resize and position
  | itself when the content changes size.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct CallOutBox<'a> {
    base:                                       Component<'a>,
    base2:                                      Timer,
    content:                                    &'a mut Component<'a>,
    outline:                                    Path,
    target_point:                               Point<f32>,
    available_area:                             Rectangle<i32>,
    target_area:                                Rectangle<i32>,
    background:                                 Image,
    arrow_size:                                 f32, // default = 16.0f
    dismissal_mouse_clicks_are_always_consumed: bool, // default = false
    creation_time:                              Time,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_CallOutBox.cpp]
impl<'a> CallOutBox<'a> {
    
    /**
      | Creates a CallOutBox.
      | 
      | -----------
      | @param contentComponent
      | 
      | the component to display inside the
      | call-out. This should already have
      | a size set (although the call-out will
      | also update itself when the component's
      | size is changed later).
      | 
      | Obviously this component must not be
      | deleted until the call-out box has been
      | deleted.
      | ----------
      | @param areaToPointTo
      | 
      | the area that the call-out's arrow should
      | point towards. If a parentComponent
      | is supplied, then this is relative to
      | that parent; otherwise, it's a global
      | screen coord.
      | ----------
      | @param parentComponent
      | 
      | if not a nullptr, this is the component
      | to add the call-out to.
      | 
      | If this is a nullptr, the call-out will
      | be added to the desktop.
      |
      */
    pub fn new(
        c:      &mut Component<'a>,
        area:   Rectangle<i32>,
        parent: *mut Component<'a>) -> Self {
    
        todo!();
        /*
        : content(c),

            addAndMakeVisible (content);

        if (parent != nullptr)
        {
            parent->addChildComponent (this);
            updatePosition (area, parent->getLocalBounds());
            setVisible (true);
        }
        else
        {
            setAlwaysOnTop (aloe_areThereAnyAlwaysOnTopWindows());
            updatePosition (area, Desktop::getInstance().getDisplays().getDisplayForRect (area)->userArea);
            addToDesktop (ComponentPeer::windowIsTemporary);

            startTimer (100);
        }

        creationTime = Time::getCurrentTime();
        */
    }

    /**
      | This will launch a callout box containing
      | the given content, pointing to the specified
      | target component.
      | 
      | This method will create and display
      | a callout, returning immediately,
      | after which the box will continue to
      | run modally until the user clicks on
      | some other component, at which point
      | it will be dismissed and deleted automatically.
      | 
      | It returns a reference to the newly-created
      | box so that you can customise it, but
      | don't keep a pointer to it, as it'll be
      | deleted at some point when it gets closed.
      | 
      | -----------
      | @param contentComponent
      | 
      | the component to display inside the
      | call-out. This should already have
      | a size set (although the call-out will
      | also update itself when the component's
      | size is changed later).
      | ----------
      | @param areaToPointTo
      | 
      | the area that the call-out's arrow should
      | point towards. If a parentComponent
      | is supplied, then this is relative to
      | that parent; otherwise, it's a global
      | screen coord.
      | ----------
      | @param parentComponent
      | 
      | if not a nullptr, this is the component
      | to add the call-out to.
      | 
      | If this is a nullptr, the call-out will
      | be added to the desktop.
      |
      */
    pub fn launch_asynchronously(
        &mut self, 
        content: Box<Component<'a>>,
        area:    Rectangle<i32>,
        parent:  *mut Component<'a>) -> &mut CallOutBox {
        
        todo!();
        /*
            jassert (content != nullptr); // must be a valid content component!

        return (new CallOutBoxCallback (std::move (content), area, parent))->callout;
        */
    }
    
    /**
      | Changes the base width of the arrow.
      |
      */
    pub fn set_arrow_size(&mut self, new_size: f32)  {
        
        todo!();
        /*
            arrowSize = newSize;
        refreshPath();
        */
    }
    
    pub fn get_border_size(&self) -> i32 {
        
        todo!();
        /*
            return jmax (getLookAndFeel().getCallOutBoxBorderSize (*this), (int) arrowSize);
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            resized();
        repaint();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawCallOutBoxBackground (*this, g, outline, background);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto borderSpace = getBorderSize();
        content.setTopLeftPosition (borderSpace, borderSpace);
        refreshPath();
        */
    }
    
    pub fn moved(&mut self)  {
        
        todo!();
        /*
            refreshPath();
        */
    }
    
    pub fn child_bounds_changed(&mut self, _0: *mut Component<'a>)  {
        
        todo!();
        /*
            updatePosition (targetArea, availableArea);
        */
    }
    
    pub fn hit_test(&mut self, x: i32, y: i32) -> bool {
        
        todo!();
        /*
            return outline.contains ((float) x, (float) y);
        */
    }
    
    pub fn input_attempt_when_modal(&mut self)  {
        
        todo!();
        /*
            if (dismissalMouseClicksAreAlwaysConsumed
             || targetArea.contains (getMouseXYRelative() + getBounds().getPosition()))
        {
            // if you click on the area that originally popped-up the callout, you expect it
            // to get rid of the box, but deleting the box here allows the click to pass through and
            // probably re-trigger it, so we need to dismiss the box asynchronously to consume the click..

            // For touchscreens, we make sure not to dismiss the CallOutBox immediately,
            // as Windows still sends touch events before the CallOutBox had a chance
            // to really open.

            auto elapsed = Time::getCurrentTime() - creationTime;

            if (elapsed.inMilliseconds() > 200)
                dismiss();
        }
        else
        {
            exitModalState (0);
            setVisible (false);
        }
        */
    }
    
    /**
      | Determines whether the mouse events
      | for clicks outside the calloutbox are
      | consumed, or allowed to arrive at the
      | other component that they were aimed
      | at.
      | 
      | By default this is false, so that when
      | you click on something outside the calloutbox,
      | that event will also be sent to the component
      | that was clicked on. If you set it to true,
      | then the first click will always just
      | dismiss the box and not be sent to anything
      | else.
      |
      */
    pub fn set_dismissal_mouse_clicks_are_always_consumed(&mut self, b: bool)  {
        
        todo!();
        /*
            dismissalMouseClicksAreAlwaysConsumed = b;
        */
    }
    
    pub fn handle_command_message(&mut self, command_id: i32)  {
        
        todo!();
        /*
            Component::handleCommandMessage (commandId);

        if (commandId == callOutBoxDismissCommandId)
        {
            exitModalState (0);
            setVisible (false);
        }
        */
    }
    
    /**
      | Posts a message which will dismiss the
      | callout box asynchronously.
      | 
      | NB: it's safe to call this method from
      | any thread.
      |
      */
    pub fn dismiss(&mut self)  {
        
        todo!();
        /*
            postCommandMessage (callOutBoxDismissCommandId);
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (key.isKeyCode (KeyPress::escapeKey))
        {
            inputAttemptWhenModal();
            return true;
        }

        return false;
        */
    }
    
    /**
      | Updates the position and size of the
      | box.
      | 
      | You shouldn't normally need to call
      | this, unless you need more precise control
      | over the layout.
      | 
      | -----------
      | @param newAreaToPointTo
      | 
      | the rectangle to make the box's arrow
      | point to
      | ----------
      | @param newAreaToFitIn
      | 
      | the area within which the box's position
      | should be constrained
      |
      */
    pub fn update_position(&mut self, 
        new_area_to_point_to: &Rectangle<i32>,
        new_area_to_fit_in:   &Rectangle<i32>)  {
        
        todo!();
        /*
            targetArea = newAreaToPointTo;
        availableArea = newAreaToFitIn;

        auto borderSpace = getBorderSize();
        auto newBounds = getLocalArea (&content, Rectangle<int> (content.getWidth()  + borderSpace * 2,
                                                                 content.getHeight() + borderSpace * 2));

        auto hw = newBounds.getWidth() / 2;
        auto hh = newBounds.getHeight() / 2;
        auto hwReduced = (float) (hw - borderSpace * 2);
        auto hhReduced = (float) (hh - borderSpace * 2);
        auto arrowIndent = (float) borderSpace - arrowSize;

        Point<float> targets[4] = { { (float) targetArea.getCentreX(), (float) targetArea.getBottom() },
                                    { (float) targetArea.getRight(),   (float) targetArea.getCentreY() },
                                    { (float) targetArea.getX(),       (float) targetArea.getCentreY() },
                                    { (float) targetArea.getCentreX(), (float) targetArea.getY() } };

        Line<float> lines[4] = { { targets[0].translated (-hwReduced, hh - arrowIndent),    targets[0].translated (hwReduced, hh - arrowIndent) },
                                 { targets[1].translated (hw - arrowIndent, -hhReduced),    targets[1].translated (hw - arrowIndent, hhReduced) },
                                 { targets[2].translated (-(hw - arrowIndent), -hhReduced), targets[2].translated (-(hw - arrowIndent), hhReduced) },
                                 { targets[3].translated (-hwReduced, -(hh - arrowIndent)), targets[3].translated (hwReduced, -(hh - arrowIndent)) } };

        auto centrePointArea = newAreaToFitIn.reduced (hw, hh).toFloat();
        auto targetCentre = targetArea.getCentre().toFloat();

        float nearest = 1.0e9f;

        for (int i = 0; i < 4; ++i)
        {
            Line<float> constrainedLine (centrePointArea.getConstrainedPoint (lines[i].getStart()),
                                         centrePointArea.getConstrainedPoint (lines[i].getEnd()));

            auto centre = constrainedLine.findNearestPointTo (targetCentre);
            auto distanceFromCentre = centre.getDistanceFrom (targets[i]);

            if (! centrePointArea.intersects (lines[i]))
                distanceFromCentre += 1000.0f;

            if (distanceFromCentre < nearest)
            {
                nearest = distanceFromCentre;
                targetPoint = targets[i];

                newBounds.setPosition ((int) (centre.x - (float) hw),
                                       (int) (centre.y - (float) hh));
            }
        }

        setBounds (newBounds);
        */
    }
    
    pub fn refresh_path(&mut self)  {
        
        todo!();
        /*
            repaint();
        background = {};
        outline.clear();

        const float gap = 4.5f;

        outline.addBubble (getLocalArea (&content, content.getLocalBounds().toFloat()).expanded (gap, gap),
                           getLocalBounds().toFloat(),
                           targetPoint - getPosition().toFloat(),
                           getLookAndFeel().getCallOutBoxCornerSize (*this), arrowSize * 0.7f);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            toFront (true);
        stopTimer();
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::window);
        */
    }
}

///-------------
#[no_copy]
pub struct CallOutBoxCallback<'a> {
    base2:   Timer,
    content: Box<Component<'a>>,
    callout: CallOutBox<'a>,
}

impl<'a> ModalComponentManagerCallback for CallOutBoxCallback<'a> {

    fn modal_state_finished(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> CallOutBoxCallback<'a> {

    pub fn new(
        c:      Box<Component<'a>>,
        area:   &Rectangle<i32>,
        parent: *mut Component<'a>) -> Self {
    
        todo!();
        /*


            : content (std::move (c)),
              callout (*content, area, parent)

            callout.setVisible (true);
            callout.enterModalState (true, this);
            startTimer (200);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (! isForegroundOrEmbeddedProcess (&callout))
                callout.dismiss();
        */
    }
}
