crate::ix!();

/**
  | Enables drag-and-drop behaviour for
  | a component and all its sub-components.
  | 
  | For a component to be able to make or receive
  | drag-and-drop events, one of its parent
  | components must derive from this class.
  | It's probably best for the top-level
  | component to implement it.
  | 
  | Then to start a drag operation, any sub-component
  | can just call the startDragging() method,
  | and this object will take over, tracking
  | the mouse and sending appropriate callbacks
  | to any child components derived from
  | DragAndDropTarget which the mouse
  | moves over.
  | 
  | -----------
  | @note
  | 
  | If all that you need to do is to respond
  | to files being drag-and-dropped from
  | the operating system onto your component,
  | you don't need any of these classes:
  | you can do this simply by overriding
  | FileDragAndDropTarget::filesDropped().
  | 
  | @see DragAndDropTarget
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct DragAndDropContainer<'a> {
    drag_image_components: Vec<DragImageComponent<'a>>,
}

impl<'a> Default for DragAndDropContainer<'a> {
    
    /**
      | Creates a DragAndDropContainer.
      | 
      | The object that derives from this class
      | must also be a Component.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<'a> DragAndDropContainer<'a> {

    #[cfg(target_os="linux")]
    pub fn perform_external_drag_drop_of_files(
        &mut self, 
        files:          &StringArray,
        can_move_files: bool,
        source_comp:    *mut Component<'a>,
        callback:       fn() -> ()

    ) -> bool {
        
        todo!();
        /*
            if (files.isEmpty())
            return false;

        if (auto* peer = getPeerForDragEvent (sourceComp))
            return XWindowSystem::getInstance()->externalDragFileInit (peer, files, canMoveFiles, std::move (callback));

        // This method must be called in response to a component's mouseDown or mouseDrag event!
        jassertfalse;
        return false;
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn perform_external_drag_drop_of_text(
        &mut self, 
        text:        &String,
        source_comp: *mut Component<'a>,
        callback:    fn() -> ()
    ) -> bool {
        
        todo!();
        /*
            if (text.isEmpty())
            return false;

        if (auto* peer = getPeerForDragEvent (sourceComp))
            return XWindowSystem::getInstance()->externalDragTextInit (peer, text, std::move (callback));

        // This method must be called in response to a component's mouseDown or mouseDrag event!
        jassertfalse;
        return false;
        */

    }

    /**
      | This performs an asynchronous drag-and-drop
      | of a set of files to some external application.
      | 
      | You can call this function in response
      | to a mouseDrag callback, and it will
      | use a native operating system drag-and-drop
      | operation to move or copy some files
      | to another application.
      | 
      | -----------
      | @param files
      | 
      | a list of filenames to drag
      | ----------
      | @param canMoveFiles
      | 
      | if true, the app that receives the files
      | is allowed to move the files to a new location
      | (if this is appropriate). If false,
      | the receiver is expected to make a copy
      | of them.
      | ----------
      | @param sourceComponent
      | 
      | normally, Aloe will assume that the
      | component under the mouse is the source
      | component of the drag, but you can use
      | this parameter to override this.
      | ----------
      | @param callback
      | 
      | an optional completion callback that
      | will be called when the operation has
      | ended.
      | 
      | -----------
      | @return
      | 
      | true if the drag operation was successfully
      | started, or false if it failed for some
      | reason
      | 
      | @see performExternalDragDropOfText
      |
      */
    pub fn perform_external_drag_drop_of_files(
        files:            &Vec<String>,
        can_move_files:   bool,
        source_component: *mut Component<'a>,
        callback:         fn() -> ()) -> bool {

        todo!();
        /*
        
        */
    }

    /**
      | This performs an asynchronous drag-and-drop
      | of a block of text to some external application.
      | 
      | You can call this function in response
      | to a mouseDrag callback, and it will
      | use a native operating system drag-and-drop
      | operation to move or copy some text to
      | another application.
      | 
      | -----------
      | @param text
      | 
      | the text to copy
      | ----------
      | @param sourceComponent
      | 
      | Normally, Aloe will assume that the
      | component under the mouse is the source
      | component of the drag, but you can use
      | this parameter to override this.
      | ----------
      | @param callback
      | 
      | an optional completion callback that
      | will be called when the operation has
      | ended.
      | 
      | -----------
      | @return
      | 
      | true if the drag operation was successfully
      | started, or false if it failed for some
      | reason
      | 
      | @see performExternalDragDropOfFiles
      |
      */
    pub fn perform_external_drag_drop_of_text(
        text:             &String,
        source_component: *mut Component<'a>,
        callback:         fn() -> ()) -> bool {

        todo!();
        /*
        
        */
    }
    
    /**
      | Begins a drag-and-drop operation.
      | 
      | This starts a drag-and-drop operation
      | - call it when the user drags the mouse
      | in your drag-source component, and
      | this object will track mouse movements
      | until the user lets go of the mouse button,
      | and will send appropriate messages
      | to DragAndDropTarget objects that
      | the mouse moves over.
      | 
      | findParentDragContainerFor() is
      | a handy method to call to find the drag
      | container to use for a component.
      | 
      | -----------
      | @param sourceDescription
      | 
      | a string or value to use as the description
      | of the thing being dragged - this will
      | be passed to the objects that might be
      | dropped-onto so they can decide whether
      | they want to handle it
      | ----------
      | @param sourceComponent
      | 
      | the component that is being dragged
      | ----------
      | @param dragImage
      | 
      | the image to drag around underneath
      | the mouse. If this is a null image, a snapshot
      | of the sourceComponent will be used
      | instead.
      | ----------
      | @param allowDraggingToOtherAloeWindows
      | 
      | if true, the dragged component will
      | appear as a desktop window, and can be
      | dragged to DragAndDropTargets that
      | are the children of components other
      | than this one.
      | ----------
      | @param imageOffsetFromMouse
      | 
      | if an image has been passed-in, this
      | specifies the offset at which the image
      | should be drawn from the mouse. If it
      | isn't specified, then the image will
      | be centred around the mouse. If an image
      | hasn't been passed-in, this will be
      | ignored.
      | ----------
      | @param inputSourceCausingDrag
      | 
      | the mouse input source which started
      | the drag. When calling from within a
      | mouseDown or mouseDrag event, you can
      | pass
      | 
      | MouseEvent::source to this method.
      | If this param is nullptr then Aloe will
      | use the mouse input source which is currently
      | dragging. If there are several dragging
      | mouse input sources (which can often
      | occur on mobile) then Aloe will use the
      | mouseInputSource which is closest
      | to the sourceComponent.
      |
      */
    pub fn start_dragging(
        &mut self, 
        source_description:                 &Var,
        source_component:                   *mut Component<'a>,
        drag_image:                         Option<Image>,
        allow_dragging_to_external_windows: Option<bool>,
        image_offset_from_mouse:            *const Point<i32>,
        input_source_causing_drag:          *const MouseInputSource)  {

        let drag_image: Image 
            = drag_image.unwrap_or(Image::default());

        let allow_dragging_to_external_windows: bool 
            = allow_dragging_to_external_windows.unwrap_or(false);
        
        todo!();
        /*
            if (isAlreadyDragging (sourceComponent))
            return;

        auto* draggingSource = getMouseInputSourceForDrag (sourceComponent, inputSourceCausingDrag);

        if (draggingSource == nullptr || ! draggingSource->isDragging())
        {
            jassertfalse;   // You must call startDragging() from within a mouseDown or mouseDrag callback!
            return;
        }

        auto lastMouseDown = draggingSource->getLastMouseDownPosition().roundToInt();
        Point<int> imageOffset;

        if (dragImage.isNull())
        {
            dragImage = sourceComponent->createComponentSnapshot (sourceComponent->getLocalBounds())
                           .convertedToFormat (Image::ARGB);

            dragImage.multiplyAllAlphas (0.6f);

            auto lo = 150;
            auto hi = 400;

            auto relPos = sourceComponent->getLocalPoint (nullptr, lastMouseDown);
            auto clipped = dragImage.getBounds().getConstrainedPoint (relPos);
            Random random;

            for (auto y = dragImage.getHeight(); --y >= 0;)
            {
                auto dy = (y - clipped.getY()) * (y - clipped.getY());

                for (auto x = dragImage.getWidth(); --x >= 0;)
                {
                    auto dx = x - clipped.getX();
                    auto distance = roundToInt (std::sqrt (dx * dx + dy));

                    if (distance > lo)
                    {
                        auto alpha = (distance > hi) ? 0
                                                     : (float) (hi - distance) / (float) (hi - lo)
                                                         + random.nextFloat() * 0.008f;

                        dragImage.multiplyAlphaAt (x, y, alpha);
                    }
                }
            }

            imageOffset = clipped;
        }
        else
        {
            if (imageOffsetFromMouse == nullptr)
                imageOffset = dragImage.getBounds().getCentre();
            else
                imageOffset = dragImage.getBounds().getConstrainedPoint (-*imageOffsetFromMouse);
        }

        auto* dragImageComponent = dragImageComponents.add (new DragImageComponent (dragImage, sourceDescription, sourceComponent,
                                                                                    draggingSource, *this, imageOffset));

        if (allowDraggingToExternalWindows)
        {
            if (! Desktop::canUseSemiTransparentWindows())
                dragImageComponent->setOpaque (true);

            dragImageComponent->addToDesktop (ComponentPeer::windowIgnoresMouseClicks
                                              | ComponentPeer::windowIsTemporary
                                              | ComponentPeer::windowIgnoresKeyPresses);
        }
        else
        {
            if (auto* thisComp = dynamic_cast<Component*> (this))
            {
                thisComp->addChildComponent (dragImageComponent);
            }
            else
            {
                jassertfalse;   // Your DragAndDropContainer needs to be a Component!
                return;
            }
        }

        dragImageComponent->sourceDetails.localPosition = sourceComponent->getLocalPoint (nullptr, lastMouseDown);
        dragImageComponent->updateLocation (false, lastMouseDown);

       #if ALOE_WINDOWS
        // Under heavy load, the layered window's paint callback can often be lost by the OS,
        // so forcing a repaint at least once makes sure that the window becomes visible..
        if (auto* peer = dragImageComponent->getPeer())
            peer->performAnyPendingRepaintsNow();
       #endif

        dragOperationStarted (dragImageComponent->sourceDetails);
        */
    }
    
    /**
      | Returns true if something is currently
      | being dragged.
      |
      */
    pub fn is_drag_and_drop_active(&self) -> bool {
        
        todo!();
        /*
            return dragImageComponents.size() > 0;
        */
    }
    
    /**
      | Returns the number of things currently
      | being dragged.
      |
      */
    pub fn get_num_current_drags(&self) -> i32 {
        
        todo!();
        /*
            return dragImageComponents.size();
        */
    }
    
    /**
      | Returns the description of the thing
      | that's currently being dragged.
      | 
      | If nothing's being dragged, this will
      | return a null var, otherwise it'll return
      | the var that was passed into startDragging().
      | 
      | If you are using drag and drop in a multi-touch
      | environment then you should use the
      | getDragDescriptionForIndex() method
      | instead which takes a touch index parameter.
      | 
      | @see startDragging, getDragDescriptionForIndex
      |
      */
    pub fn get_current_drag_description(&self) -> Var {
        
        todo!();
        /*
            // If you are performing drag and drop in a multi-touch environment then
        // you should use the getDragDescriptionForIndex() method instead!
        jassert (dragImageComponents.size() < 2);

        return dragImageComponents.size() != 0 ? dragImageComponents[0]->sourceDetails.description
                                               : var();
        */
    }
    
    /**
      | Same as the getCurrentDragDescription()
      | method but takes a touch index parameter.
      | 
      | @see getCurrentDragDescription
      |
      */
    pub fn get_drag_description_for_index(&self, index: i32) -> Var {
        
        todo!();
        /*
            if (! isPositiveAndBelow (index, dragImageComponents.size()))
            return {};

        return dragImageComponents.getUnchecked (index)->sourceDetails.description;
        */
    }
    
    /**
      | If a drag is in progress, this allows
      | the image being shown to be dynamically
      | updated.
      | 
      | If you are using drag and drop in a multi-touch
      | environment then you should use the
      | setDragImageForIndex() method instead
      | which takes a touch index parameter.
      | 
      | @see setDragImageForIndex
      |
      */
    pub fn set_current_drag_image(&mut self, new_image: &Image)  {
        
        todo!();
        /*
            // If you are performing drag and drop in a multi-touch environment then
        // you should use the setDragImageForIndex() method instead!
        jassert (dragImageComponents.size() < 2);

        dragImageComponents[0]->updateImage (newImage);
        */
    }
    
    /**
      | Same as the setCurrentDragImage()
      | method but takes a touch index parameter.
      | 
      | @see setCurrentDragImage
      |
      */
    pub fn set_drag_image_for_index(&mut self, 
        index:     i32,
        new_image: &Image)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, dragImageComponents.size()))
            dragImageComponents.getUnchecked (index)->updateImage (newImage);
        */
    }
    
    /**
      | Utility to find the DragAndDropContainer
      | for a given Component.
      | 
      | This will search up this component's
      | parent hierarchy looking for the first
      | parent component which is a DragAndDropContainer.
      | 
      | It's useful when a component wants to
      | call startDragging but doesn't know
      | the DragAndDropContainer it should
      | to use.
      | 
      | Obviously this may return nullptr if
      | it doesn't find a suitable component.
      |
      */
    pub fn find_parent_drag_container_for(&mut self, c: *mut Component<'a>) -> *mut DragAndDropContainer {
        
        todo!();
        /*
            return c != nullptr ? c->findParentComponentOfClass<DragAndDropContainer>() : nullptr;
        */
    }
    
    pub fn should_drop_files_when_dragged_externally(
        &mut self, 
        _0: &DragAndDropTargetSourceDetails,
        _1: &mut Vec<String>,
        _2: &mut bool) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn should_drop_text_when_dragged_externally(
        &mut self, 
        _0: &DragAndDropTargetSourceDetails,
        _1: &mut String) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn drag_operation_started(&mut self, _0: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn drag_operation_ended(&mut self, _0: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_mouse_input_source_for_drag(
        &mut self, 
        source_component:          *mut Component<'a>,
        input_source_causing_drag: *const MouseInputSource) -> *const MouseInputSource {
        
        todo!();
        /*
            if (inputSourceCausingDrag == nullptr)
        {
            auto minDistance = std::numeric_limits<float>::max();
            auto& desktop = Desktop::getInstance();

            auto centrePoint = sourceComponent ? sourceComponent->getScreenBounds().getCentre().toFloat() : Point<float>();
            auto numDragging = desktop.getNumDraggingMouseSources();

            for (auto i = 0; i < numDragging; ++i)
            {
                if (auto* ms = desktop.getDraggingMouseSource (i))
                {
                    auto distance =  ms->getScreenPosition().getDistanceSquaredFrom (centrePoint);

                    if (distance < minDistance)
                    {
                        minDistance = distance;
                        inputSourceCausingDrag = ms;
                    }
                }
            }
        }

        // You must call startDragging() from within a mouseDown or mouseDrag callback!
        jassert (inputSourceCausingDrag != nullptr && inputSourceCausingDrag->isDragging());

        return inputSourceCausingDrag;
        */
    }
    
    pub fn is_already_dragging(&self, component: *mut Component<'a>) -> bool {
        
        todo!();
        /*
            for (auto* dragImageComp : dragImageComponents)
        {
            if (dragImageComp->sourceDetails.sourceComponent == component)
                return true;
        }

        return false;
        */
    }
}
