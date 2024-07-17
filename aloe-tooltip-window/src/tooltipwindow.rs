crate::ix!();

#[cfg(ALOE_DEBUG)]
lazy_static!{
    /*
    static Vec<TooltipWindow*> activeTooltipWindows;
    */
}

pub trait GetTipFor {

    /**
      | Asks a component for its tooltip.
      | 
      | This can be overridden if you need custom
      | lookup behaviour or to modify the strings.
      |
      */
    fn get_tip_for(&mut self, _0: &mut Component<'_>) -> String;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_TooltipWindow.h]

/**
  | A window that displays a pop-up tooltip
  | when the mouse hovers over another component.
  | 
  | To enable tooltips in your app, just
  | create a single instance of a TooltipWindow
  | object. Note that if you instantiate
  | more than one instance of this class
  | with the same parentComponent (even
  | if both TooltipWindow's parentComponent
  | is nil), you'll end up with multiple
  | tooltips being shown! To avoid this
  | use a SharedResourcePointer to instantiate
  | the TooltipWindow only once.
  | 
  | For audio plug-ins (which should not
  | be opening native windows) it is better
  | to add a TooltipWindow as a member variable
  | to the editor and ensure that the editor
  | is the parentComponent of your TooltipWindow.
  | This will ensure that your
  | 
  | TooltipWindow is scaled according
  | to your editor and the DAWs scaling setting.
  | 
  | The TooltipWindow object will then
  | stay invisible, waiting until the mouse
  | hovers for the specified length of time
  | - it will then see if it's currently over
  | a component which implements the TooltipClient
  | interface, and if so, it will make itself
  | visible to show the tooltip in the appropriate
  | place.
  | 
  | @see TooltipClient, SettableTooltipClient,
  | SharedResourcePointer
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TooltipWindow<'a> {
    base:                            Component<'a>,
    base2:                           Timer,
    last_mouse_pos:                  Point<f32>,
    last_component_under_mouse:      *mut Component<'a>, // default = nullptr
    tip_showing:                     String,
    last_tip_under_mouse:            String,
    milliseconds_before_tip_appears: i32,
    mouse_clicks:                    i32, // default = 0
    mouse_wheel_moves:               i32, // default = 0
    last_comp_change_time:           u32, // default = 0
    last_hide_time:                  u32, // default = 0
    reentrant:                       bool, // default = false
}

pub trait TooltipWindowInterface: GetTipFor {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_TooltipWindow.cpp]
impl<'a> Drop for TooltipWindow<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            hideTip();
        */
    }
}

impl<'a> TooltipWindow<'a> {

    /**
      | Creates a tooltip window.
      | 
      | Make sure your app only creates one instance
      | of this class, otherwise you'll get
      | multiple overlaid tooltips appearing.
      | The window will initially be invisible
      | and will make itself visible when it
      | needs to display a tip.
      | 
      | To change the style of tooltips, see
      | the LookAndFeel class for its tooltip
      | methods.
      | 
      | -----------
      | @param parentComponent
      | 
      | if set to nullptr, the TooltipWindow
      | will appear on the desktop, otherwise
      | the tooltip will be added to the given
      | parent component.
      | ----------
      | @param millisecondsBeforeTipAppears
      | 
      | the time for which the mouse has to stay
      | still before a tooltip will be shown
      | 
      | @see TooltipClient, LookAndFeel::drawTooltip,
      | LookAndFeel::getTooltipBounds
      |
      */
    pub fn new(
        parent_comp: *mut Component<'a>,
        delay_ms:    Option<i32>

    ) -> Self {

        let delay_ms: i32 = delay_ms.unwrap_or(700);
    
        todo!();
        /*
        : component("tooltip"),
        : milliseconds_before_tip_appears(delayMs),

            setAlwaysOnTop (true);
        setOpaque (true);
        setAccessible (false);

        if (parentComp != nullptr)
            parentComp->addChildComponent (this);

        if (Desktop::getInstance().getMainMouseSource().canHover())
            startTimer (123);
        */
    }
    
    /**
      | Changes the time before the tip appears.
      | 
      | This lets you change the value that was
      | set in the constructor.
      |
      */
    pub fn set_milliseconds_before_tip_appears(
        &mut self, 
        new_time_ms: Option<i32>

    ) {

        let new_time_ms: i32 = new_time_ms.unwrap_or(700);
        
        todo!();
        /*
            millisecondsBeforeTipAppears = newTimeMs;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawTooltip (g, tipShowing, getWidth(), getHeight());
        */
    }
    
    pub fn mouse_enter(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            hideTip();
        */
    }
    
    pub fn update_position(&mut self, 
        tip:         &String,
        pos:         Point<i32>,
        parent_area: Rectangle<i32>)  {
        
        todo!();
        /*
            setBounds (getLookAndFeel().getTooltipBounds (tip, pos, parentArea));
        setVisible (true);
        */
    }
    
    /**
      | Can be called to manually force a tip
      | to be shown at a particular location.
      |
      */
    pub fn display_tip(&mut self, 
        screen_pos: Point<i32>,
        tip:        &String)  {
        
        todo!();
        /*
            jassert (tip.isNotEmpty());

        if (! reentrant)
        {
            ScopedValueSetter<bool> setter (reentrant, true, false);

            if (tipShowing != tip)
            {
                tipShowing = tip;
                repaint();
            }

            if (auto* parent = getParentComponent())
            {
                updatePosition (tip, parent->getLocalPoint (nullptr, screenPos),
                                parent->getLocalBounds());
            }
            else
            {
                const auto physicalPos = ScalingHelpers::scaledScreenPosToUnscaled (screenPos);
                const auto scaledPos = ScalingHelpers::unscaledScreenPosToScaled (*this, physicalPos);
                updatePosition (tip, scaledPos, Desktop::getInstance().getDisplays().getDisplayForPoint (screenPos)->userArea);

                addToDesktop (ComponentPeer::windowHasDropShadow
                              | ComponentPeer::windowIsTemporary
                              | ComponentPeer::windowIgnoresKeyPresses
                              | ComponentPeer::windowIgnoresMouseClicks);
            }

           #if ALOE_DEBUG
            activeTooltipWindows.addIfNotAlreadyThere (this);

            auto* parent = getParentComponent();

            for (auto* w : activeTooltipWindows)
            {
                if (w != nullptr && w != this && w->tipShowing == tipShowing && w->getParentComponent() == parent)
                {
                    // Looks like you have more than one TooltipWindow showing the same tip..
                    // Be careful not to create more than one instance of this class with the
                    // same parent component!
                    jassertfalse;
                }
            }
           #endif

            toFront (false);
        }
        */
    }
    
    pub fn get_tip_for(&mut self, c: &mut Component<'a>) -> String {
        
        todo!();
        /*
            if (isForegroundOrEmbeddedProcess (&c)
             && ! ModifierKeys::currentModifiers.isAnyMouseButtonDown())
        {
            if (auto* ttc = dynamic_cast<TooltipClient*> (&c))
                if (! c.isCurrentlyBlockedByAnotherModalComponent())
                    return ttc->getTooltip();
        }

        return {};
        */
    }
    
    /**
      | Can be called to manually hide the tip
      | if it's showing.
      |
      */
    pub fn hide_tip(&mut self)  {
        
        todo!();
        /*
            if (! reentrant)
        {
            tipShowing.clear();
            removeFromDesktop();
            setVisible (false);

           #if ALOE_DEBUG
            activeTooltipWindows.removeAllInstancesOf (this);
           #endif
        }
        */
    }
    
    pub fn get_desktop_scale_factor(&self) -> f32 {
        
        todo!();
        /*
            if (lastComponentUnderMouse != nullptr)
            return Component::getApproximateScaleFactorForComponent (lastComponentUnderMouse);

        return Component::getDesktopScaleFactor();
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return createIgnoredAccessibilityHandler (*this);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto& desktop = Desktop::getInstance();
        auto mouseSource = desktop.getMainMouseSource();
        auto now = Time::getApproximateMillisecondCounter();

        auto* newComp = mouseSource.isTouch() ? nullptr : mouseSource.getComponentUnderMouse();

        if (newComp == nullptr || getParentComponent() == nullptr || newComp->getPeer() == getPeer())
        {
            auto newTip = newComp != nullptr ? getTipFor (*newComp) : String();
            bool tipChanged = (newTip != lastTipUnderMouse || newComp != lastComponentUnderMouse);
            lastComponentUnderMouse = newComp;
            lastTipUnderMouse = newTip;

            auto clickCount = desktop.getMouseButtonClickCounter();
            auto wheelCount = desktop.getMouseWheelMoveCounter();
            bool mouseWasClicked = (clickCount > mouseClicks || wheelCount > mouseWheelMoves);
            mouseClicks = clickCount;
            mouseWheelMoves = wheelCount;

            auto mousePos = mouseSource.getScreenPosition();
            bool mouseMovedQuickly = mousePos.getDistanceFrom (lastMousePos) > 12;
            lastMousePos = mousePos;

            if (tipChanged || mouseWasClicked || mouseMovedQuickly)
                lastCompChangeTime = now;

            if (isVisible() || now < lastHideTime + 500)
            {
                // if a tip is currently visible (or has just disappeared), update to a new one
                // immediately if needed..
                if (newComp == nullptr || mouseWasClicked || newTip.isEmpty())
                {
                    if (isVisible())
                    {
                        lastHideTime = now;
                        hideTip();
                    }
                }
                else if (tipChanged)
                {
                    displayTip (mousePos.roundToInt(), newTip);
                }
            }
            else
            {
                // if there isn't currently a tip, but one is needed, only let it
                // appear after a timeout..
                if (newTip.isNotEmpty()
                     && newTip != tipShowing
                     && now > lastCompChangeTime + (uint32) millisecondsBeforeTipAppears)
                {
                    displayTip (mousePos.roundToInt(), newTip);
                }
            }
        }
        */
    }
}
