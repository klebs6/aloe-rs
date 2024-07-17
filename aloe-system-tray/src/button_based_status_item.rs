crate::ix!();

pub struct ButtonBasedStatusItem<'a,StatusItem:NSStatusItem,ImageType:NSImage> {
    base:            StatusItemContainer<'a,StatusItem,ImageType>,
    event_forwarder: NSUniquePtr<NSObject>,
}

impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> ButtonBasedStatusItem<'a,StatusItem,ImageType> {

    pub fn new(
        icon_comp: &mut SystemTrayIconComponent<'a,StatusItem,ImageType>,
        im:        &Image) -> Self {

        todo!();
        /*
        : status_item_container(iconComp, im),

            static ButtonEventForwarderClass cls;
            eventForwarder.reset ([cls.createInstance() init]);
            ButtonEventForwarderClass::setOwner (eventForwarder.get(), this);

            setIconSize();
            configureIcon();

            statusItem.reset ([[[NSStatusBar systemStatusBar] statusItemWithLength: NSSquareStatusItemLength] retain]);
            auto button = [statusItem.get() button];
            button.image = statusIcon.get();
            button.target = eventForwarder.get();
            button.action = @selector (handleEvent:);
           #if defined (MAC_OS_X_VERSION_10_12) && MAC_OS_X_VERSION_MAX_ALLOWED >= MAC_OS_X_VERSION_10_12
            [button sendActionOn: NSEventMaskLeftMouseDown | NSEventMaskRightMouseDown | NSEventMaskScrollWheel];
           #else
            [button sendActionOn: NSLeftMouseDownMask | NSRightMouseDownMask | NSScrollWheelMask];
           #endif
        */
    }
    
    pub fn configure_icon(&mut self)  {
        
        todo!();
        /*
            [statusIcon.get() setTemplate: true];
            [statusItem.get() button].image = statusIcon.get();
        */
    }
    
    pub fn set_highlighted(&mut self, should_highlight: bool)  {
        
        todo!();
        /*
            [[statusItem.get() button] setHighlighted: shouldHighlight];
        */
    }
    
    pub fn handle_event(&mut self)  {
        
        todo!();
        /*
            auto e = [NSApp currentEvent];
            NSEventType type = [e type];

            const bool isLeft  = (type == NSEventTypeLeftMouseDown);
            const bool isRight = (type == NSEventTypeRightMouseDown);

            if (owner.isCurrentlyBlockedByAnotherModalComponent())
            {
                if (isLeft || isRight)
                    if (auto* current = Component::getCurrentlyModalComponent())
                        current->inputAttemptWhenModal();
            }
            else
            {
                auto eventMods = ComponentPeer::getCurrentModifiersRealtime();

                if (([e modifierFlags] & NSEventModifierFlagCommand) != 0)
                    eventMods = eventMods.withFlags (ModifierKeys::commandModifier);

                auto now = Time::getCurrentTime();
                auto mouseSource = Desktop::getInstance().getMainMouseSource();
                auto pressure = (float) e.pressure;

                if (isLeft || isRight)
                {
                    owner.mouseDown ({ mouseSource, {},
                                       eventMods.withFlags (isLeft ? ModifierKeys::leftButtonModifier
                                                                   : ModifierKeys::rightButtonModifier),
                                       pressure,
                                       MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                                       MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                                       &owner, &owner, now, {}, now, 1, false });

                    owner.mouseUp   ({ mouseSource, {},
                                       eventMods.withoutMouseButtons(),
                                       pressure,
                                       MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                                       MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                                       &owner, &owner, now, {}, now, 1, false });
                }
                else if (type == NSEventTypeMouseMoved)
                {
                    owner.mouseMove (MouseEvent (mouseSource, {}, eventMods, pressure,
                                                 MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                                                 MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                                                 &owner, &owner, now, {}, now, 1, false));
                }
            }
        */
    }
}
