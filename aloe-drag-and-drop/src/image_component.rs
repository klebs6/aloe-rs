crate::ix!();

#[no_copy]
pub struct DragImageComponent<'a> {
    base:                          Component<'a>,
    base2:                         Timer,
    source_details:                DragAndDropTargetSourceDetails<'a>,
    image:                         Image,
    owner:                         &'a mut DragAndDropContainer<'a>,
    mouse_drag_source:             WeakReference<Component<'a>>,
    currently_over_comp:           WeakReference<Component<'a>>,
    image_offset:                  Point<i32>,
    has_checked_for_external_drag: bool, // default = false
    last_time_over_target:         Time,
    original_input_source_index:   i32,
    original_input_source_type:    MouseInputSourceType,
}

impl<'a> Drop for DragImageComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                owner.dragImageComponents.remove (owner.dragImageComponents.indexOf (this), false);

                if (mouseDragSource != nullptr)
                {
                    mouseDragSource->removeMouseListener (this);

                    if (auto* current = getCurrentlyOver())
                        if (current->isInterestedInDragSource (sourceDetails))
                            current->itemDragExit (sourceDetails);
                }

                owner.dragOperationEnded (sourceDetails);
             */
    }
}

impl<'a> DragImageComponent<'a> {

    pub fn new(
        im:               &Image,
        desc:             &Var,
        source_component: *mut Component<'a>,
        dragging_source:  *const MouseInputSource,
        ddc:              &mut DragAndDropContainer,
        offset:           Point<i32>) -> Self {
    
        todo!();
        /*


            : sourceDetails (desc, sourceComponent, Point<int>()),
                  image (im), owner (ddc),
                  mouseDragSource (draggingSource->getComponentUnderMouse()),
                  imageOffset (offset),
                  originalInputSourceIndex (draggingSource->getIndex()),
                  originalInputSourceType (draggingSource->getType())
                updateSize();

                if (mouseDragSource == nullptr)
                    mouseDragSource = sourceComponent;

                mouseDragSource->addMouseListener (this, false);

                startTimer (200);

                setInterceptsMouseClicks (false, false);
                setAlwaysOnTop (true);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (isOpaque())
                    g.fillAll (Colours::white);

                g.setOpacity (1.0f);
                g.drawImageAt (image, 0, 0);
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.originalComponent != this && isOriginalInputSource (e.source))
                {
                    if (mouseDragSource != nullptr)
                        mouseDragSource->removeMouseListener (this);

                    // (note: use a local copy of this in case the callback runs
                    // a modal loop and deletes this object before the method completes)
                    auto details = sourceDetails;
                    DragAndDropTarget* finalTarget = nullptr;

                    auto wasVisible = isVisible();
                    setVisible (false);
                    Component* unused;
                    finalTarget = findTarget (e.getScreenPosition(), details.localPosition, unused);

                    if (wasVisible) // fade the component and remove it - it'll be deleted later by the timer callback
                        dismissWithAnimation (finalTarget == nullptr);

                    if (auto* parent = getParentComponent())
                        parent->removeChildComponent (this);

                    if (finalTarget != nullptr)
                    {
                        currentlyOverComp = nullptr;
                        finalTarget->itemDropped (details);
                    }

                    // careful - this object could now be deleted..
                }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.originalComponent != this && isOriginalInputSource (e.source))
                    updateLocation (true, e.getScreenPosition());
        */
    }
    
    pub fn update_location(
        &mut self, 
        can_do_external_drag: bool,
        screen_pos:           Point<i32>

    ) {
        
        todo!();
        /*
            auto details = sourceDetails;

                setNewScreenPos (screenPos);

                Component* newTargetComp;
                auto* newTarget = findTarget (screenPos, details.localPosition, newTargetComp);

                setVisible (newTarget == nullptr || newTarget->shouldDrawDragImageWhenOver());

                if (newTargetComp != currentlyOverComp)
                {
                    if (auto* lastTarget = getCurrentlyOver())
                        if (details.sourceComponent != nullptr && lastTarget->isInterestedInDragSource (details))
                            lastTarget->itemDragExit (details);

                    currentlyOverComp = newTargetComp;

                    if (newTarget != nullptr
                          && newTarget->isInterestedInDragSource (details))
                        newTarget->itemDragEnter (details);
                }

                sendDragMove (details);

                if (canDoExternalDrag)
                {
                    auto now = Time::getCurrentTime();

                    if (getCurrentlyOver() != nullptr)
                        lastTimeOverTarget = now;
                    else if (now > lastTimeOverTarget + RelativeTime::milliseconds (700))
                        checkForExternalDrag (details, screenPos);
                }

                forceMouseCursorUpdate();
        */
    }
    
    pub fn update_image(&mut self, new_image: &Image)  {
        
        todo!();
        /*
            image = newImage;
                updateSize();
                repaint();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            forceMouseCursorUpdate();

                if (sourceDetails.sourceComponent == nullptr)
                {
                    deleteSelf();
                }
                else
                {
                    for (auto& s : Desktop::getInstance().getMouseSources())
                    {
                        if (isOriginalInputSource (s) && ! s.isDragging())
                        {
                            if (mouseDragSource != nullptr)
                                mouseDragSource->removeMouseListener (this);

                            deleteSelf();
                            break;
                        }
                    }
                }
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (key == KeyPress::escapeKey)
                {
                    dismissWithAnimation (true);
                    deleteSelf();
                    return true;
                }

                return false;
        */
    }
    
    pub fn can_modal_event_be_sent_to_component(
        &mut self, 
        target_component: *const Component<'a>

    ) -> bool {
        
        todo!();
        /*
            return targetComponent == mouseDragSource;
        */
    }

    /**
      | (overridden to avoid beeps when dragging)
      |
      */
    pub fn input_attempt_when_modal(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn update_size(&mut self)  {
        
        todo!();
        /*
            setSize (image.getWidth(), image.getHeight());
        */
    }
    
    pub fn force_mouse_cursor_update(&mut self)  {
        
        todo!();
        /*
            Desktop::getInstance().getMainMouseSource().forceMouseCursorUpdate();
        */
    }
    
    pub fn get_currently_over(&self) 
        -> Option<&mut dyn DragAndDropTarget> 
    {
        todo!();

        /*
            return dynamic_cast<DragAndDropTarget*> (currentlyOverComp.get());
        */
    }
    
    pub fn find_desktop_component_below(
        &'a self,
        screen_pos: Point<i32>
    ) 
        -> Option<&'a mut Component<'a>>
    {
        
        todo!();
        /*
            auto& desktop = Desktop::getInstance();

                for (auto i = desktop.getNumComponents(); --i >= 0;)
                {
                    auto* desktopComponent = desktop.getComponent (i);
                    auto dPoint = desktopComponent->getLocalPoint (nullptr, screenPos);

                    if (auto* c = desktopComponent->getComponentAt (dPoint))
                    {
                        auto cPoint = c->getLocalPoint (desktopComponent, dPoint);

                        if (c->hitTest (cPoint.getX(), cPoint.getY()))
                            return c;
                    }
                }

                return nullptr;
        */
    }
    
    pub fn find_target(
        &self, 
        screen_pos:       Point<i32>,
        relative_pos:     &mut Point<i32>,
        result_component: &mut *mut Component<'a>

    ) 
        -> Option<&mut dyn DragAndDropTarget> 
    {
        
        todo!();
        /*
            auto* hit = getParentComponent();

                if (hit == nullptr)
                    hit = findDesktopComponentBelow (screenPos);
                else
                    hit = hit->getComponentAt (hit->getLocalPoint (nullptr, screenPos));

                // (note: use a local copy of this in case the callback runs
                // a modal loop and deletes this object before the method completes)
                auto details = sourceDetails;

                while (hit != nullptr)
                {
                    if (auto* ddt = dynamic_cast<DragAndDropTarget*> (hit))
                    {
                        if (ddt->isInterestedInDragSource (details))
                        {
                            relativePos = hit->getLocalPoint (nullptr, screenPos);
                            resultComponent = hit;
                            return ddt;
                        }
                    }

                    hit = hit->getParentComponent();
                }

                resultComponent = nullptr;
                return nullptr;
        */
    }
    
    pub fn set_new_screen_pos(&mut self, screen_pos: Point<i32>)  {
        
        todo!();
        /*
            auto newPos = screenPos - imageOffset;

                if (auto* p = getParentComponent())
                    newPos = p->getLocalPoint (nullptr, newPos);

                setTopLeftPosition (newPos);
        */
    }
    
    pub fn send_drag_move(&self, details: &mut DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            if (auto* target = getCurrentlyOver())
                    if (target->isInterestedInDragSource (details))
                        target->itemDragMove (details);
        */
    }
    
    pub fn check_for_external_drag(
        &mut self, 
        details:    &mut DragAndDropTargetSourceDetails,
        screen_pos: Point<i32>

    ) {
        
        todo!();
        /*
            if (! hasCheckedForExternalDrag)
                {
                    if (Desktop::getInstance().findComponentAt (screenPos) == nullptr)
                    {
                        hasCheckedForExternalDrag = true;

                        if (ComponentPeer::getCurrentModifiersRealtime().isAnyMouseButtonDown())
                        {
                            Vec<String> files;
                            auto canMoveFiles = false;

                            if (owner.shouldDropFilesWhenDraggedExternally (details, files, canMoveFiles) && ! files.isEmpty())
                            {
                                MessageManager::callAsync ([=] { DragAndDropContainer::performExternalDragDropOfFiles (files, canMoveFiles); });
                                deleteSelf();
                                return;
                            }

                            String text;

                            if (owner.shouldDropTextWhenDraggedExternally (details, text) && text.isNotEmpty())
                            {
                                MessageManager::callAsync ([=] { DragAndDropContainer::performExternalDragDropOfText (text); });
                                deleteSelf();
                                return;
                            }
                        }
                    }
                }
        */
    }
    
    pub fn delete_self(&mut self)  {
        
        todo!();
        /*
            delete this;
        */
    }
    
    pub fn dismiss_with_animation(&mut self, should_snap_back: bool)  {
        
        todo!();
        /*
            setVisible (true);
                auto& animator = Desktop::getInstance().getAnimator();

                if (shouldSnapBack && sourceDetails.sourceComponent != nullptr)
                {
                    auto target = sourceDetails.sourceComponent->localPointToGlobal (sourceDetails.sourceComponent->getLocalBounds().getCentre());
                    auto ourCentre = localPointToGlobal (getLocalBounds().getCentre());

                    animator.animateComponent (this,
                                               getBounds() + (target - ourCentre),
                                               0.0f, 120,
                                               true, 1.0, 1.0);
                }
                else
                {
                    animator.fadeOut (this, 120);
                }
        */
    }
    
    pub fn is_original_input_source(
        &mut self, 
        source_to_check: &MouseInputSource

    ) -> bool {
        
        todo!();
        /*
            return (sourceToCheck.getType() == originalInputSourceType
                        && sourceToCheck.getIndex() == originalInputSourceIndex);
        */
    }
}
