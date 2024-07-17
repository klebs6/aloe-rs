crate::ix!();

pub struct ViewBasedStatusItem<'a,Control: NSControl,StatusItem:NSStatusItem,ImageType:NSImage> {
    base:           StatusItemContainer<'a,StatusItem,ImageType>,
    view:           NSUniquePtr<Box<Control>>,
    is_highlighted: bool, // default = false
}

impl<'a,Control:NSControl,StatusItem:NSStatusItem,ImageType:NSImage> Drop for ViewBasedStatusItem<'a,Control,StatusItem,ImageType> {

    fn drop(&mut self) {
        todo!();
        /*
            [[NSNotificationCenter defaultCenter] removeObserver: view.get()];
            [[NSStatusBar systemStatusBar] removeStatusItem: statusItem.get()];
            SystemTrayViewClass::setOwner (view.get(), nullptr);
            SystemTrayViewClass::setImage (view.get(), nil);
        */
    }
}

impl<'a,Control:NSControl,StatusItem:NSStatusItem,ImageType:NSImage> ViewBasedStatusItem<'a,Control,StatusItem,ImageType> {

    pub fn new(
        icon_comp: &mut SystemTrayIconComponent<'a,StatusItem,ImageType>,
        im:        &ImageType) -> Self {
    
        todo!();
        /*
        : status_item_container(iconComp, im),

            static SystemTrayViewClass cls;
            view.reset ([cls.createInstance() init]);
            SystemTrayViewClass::setOwner (view.get(), this);
            SystemTrayViewClass::setImage (view.get(), statusIcon.get());

            setIconSize();

            statusItem.reset ([[[NSStatusBar systemStatusBar] statusItemWithLength: NSSquareStatusItemLength] retain]);
            [statusItem.get() setView: view.get()];

            SystemTrayViewClass::frameChanged (view.get(), SEL(), nullptr);

            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
            [[NSNotificationCenter defaultCenter]  addObserver: view.get()
                                                      selector: @selector (frameChanged:)
                                                          name: NSWindowDidMoveNotification
                                                        object: nil];
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE
        */
    }
    
    pub fn configure_icon(&mut self)  {
        
        todo!();
        /*
            SystemTrayViewClass::setImage (view.get(), statusIcon.get());
            [statusItem.get() setView: view.get()];
        */
    }
    
    pub fn set_highlighted(&mut self, should_highlight: bool)  {
        
        todo!();
        /*
            isHighlighted = shouldHighlight;
            [view.get() setNeedsDisplay: true];
        */
    }
    
    pub fn handle_status_item_action<Event:NSEvent>(&mut self, e: *mut Event)  {
        
        todo!();
        /*
            NSEventType type = [e type];

            const bool isLeft  = (type == NSEventTypeLeftMouseDown  || type == NSEventTypeLeftMouseUp);
            const bool isRight = (type == NSEventTypeRightMouseDown || type == NSEventTypeRightMouseUp);

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

                if (isLeft || isRight)  // Only mouse up is sent by the OS, so simulate a down/up
                {
                    setHighlighted (true);
                    startTimer (150);

                    owner.mouseDown (MouseEvent (mouseSource, {},
                                                 eventMods.withFlags (isLeft ? ModifierKeys::leftButtonModifier
                                                                             : ModifierKeys::rightButtonModifier),
                                                 pressure, MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                                                 MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                                                 &owner, &owner, now, {}, now, 1, false));

                    owner.mouseUp (MouseEvent (mouseSource, {}, eventMods.withoutMouseButtons(), pressure,
                                               MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                                               MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                                               &owner, &owner, now, {}, now, 1, false));
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
