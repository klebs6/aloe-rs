crate::ix!();

pub struct DemoThumbnailComp<'a> {
    base:                    Component<'a>,
    base4:                   ChangeBroadcaster<'a>,
    base6:                   Timer,
    transport_source:        &'a mut AudioTransportSource<'a>,
    zoom_slider:             &'a mut Slider<'a>,
    scrollbar:               ScrollBar<'a>,           // default = false 
    thumbnail_cache:         AudioThumbnailCache, // default = 5 
    thumbnail:               AudioThumbnail<'a>,
    visible_range:           Range<f64>,
    is_following_transport:  bool,                // default = false
    last_file_dropped:       Url,
    current_position_marker: DrawableRectangle<'a>,
}

impl<'a> FileDragAndDropTarget for DemoThumbnailComp<'a> {

}

impl<'a> ItemDropped for DemoThumbnailComp<'a> {

    fn item_dropped(&mut self, _: &DragAndDropTargetSourceDetails<'_>) { 
        todo!() 
    }
}

impl<'a> ItemDragMove for DemoThumbnailComp<'a> {

}

impl<'a> ItemDragEnter for DemoThumbnailComp<'a> {

}

impl<'a> IsInterestedInDragSource for DemoThumbnailComp<'a> {

    fn is_interested_in_drag_source(&mut self, _: &DragAndDropTargetSourceDetails<'_>) -> bool { 
        todo!() 
    }
}

impl<'a> ItemDragExit for DemoThumbnailComp<'a> {

}

impl<'a> ShouldDrawDragImageWhenOver for DemoThumbnailComp<'a> {

}

impl<'a> ScrollBarListener for DemoThumbnailComp<'a> {

    fn scroll_bar_moved(
        &mut self, 
        scroll_bar_that_has_moved: *mut ScrollBar,
        new_range_start:           f64

    ) {
        
        todo!();
        /*
            if (scrollBarThatHasMoved == &scrollbar)
                if (! (isFollowingTransport && transportSource.isPlaying()))
                    setRange (visibleRange.movedToStartAt (newRangeStart));
        */
    }
}

impl<'a> ChangeListener for DemoThumbnailComp<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            // this method is called by the thumbnail when it has changed, so we should repaint it..
            repaint();
        */
    }
}

impl<'a> Drop for DemoThumbnailComp<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            scrollbar.removeListener (this);
            thumbnail.removeChangeListener (this);
         */
    }
}

impl<'a> DemoThumbnailComp<'a> {

    pub fn new(
        format_manager: &mut AudioFormatManager,
        source:         &mut AudioTransportSource,
        slider:         &mut Slider) -> Self {
    
        todo!();
        /*
        : transport_source(source),
        : zoom_slider(slider),
        : thumbnail(512, formatManager, thumbnailCache),

            thumbnail.addChangeListener (this);

            addAndMakeVisible (scrollbar);
            scrollbar.setRangeLimits (visibleRange);
            scrollbar.setAutoHide (false);
            scrollbar.addListener (this);

            currentPositionMarker.setFill (Colours::white.withAlpha (0.85f));
            addAndMakeVisible (currentPositionMarker);
        */
    }
    
    pub fn seturl(&mut self, url: &Url)  {
        
        todo!();
        /*
            InputSource* inputSource = nullptr;

           #if ! ALOE_IOS
            if (url.isLocalFile())
            {
                inputSource = new FileInputSource (url.getLocalFile());
            }
            else
           #endif
            {
                if (inputSource == nullptr)
                    inputSource = new URLInputSource (url);
            }

            if (inputSource != nullptr)
            {
                thumbnail.setSource (inputSource);

                Range<double> newRange (0.0, thumbnail.getTotalLength());
                scrollbar.setRangeLimits (newRange);
                setRange (newRange);

                startTimerHz (40);
            }
        */
    }
    
    pub fn get_last_dropped_file(&self) -> Url {
        
        todo!();
        /*
            return lastFileDropped;
        */
    }
    
    pub fn set_zoom_factor(&mut self, amount: f64)  {
        
        todo!();
        /*
            if (thumbnail.getTotalLength() > 0)
            {
                auto newScale = jmax (0.001, thumbnail.getTotalLength() * (1.0 - jlimit (0.0, 0.99, amount)));
                auto timeAtCentre = xToTime ((float) getWidth() / 2.0f);

                setRange ({ timeAtCentre - newScale * 0.5, timeAtCentre + newScale * 0.5 });
            }
        */
    }
    
    pub fn set_range(&mut self, new_range: Range<f64>)  {
        
        todo!();
        /*
            visibleRange = newRange;
            scrollbar.setCurrentRange (visibleRange);
            updateCursorPosition();
            repaint();
        */
    }
    
    pub fn set_follows_transport(&mut self, should_follow: bool)  {
        
        todo!();
        /*
            isFollowingTransport = shouldFollow;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::darkgrey);
            g.setColour (Colours::lightblue);

            if (thumbnail.getTotalLength() > 0.0)
            {
                auto thumbArea = getLocalBounds();

                thumbArea.removeFromBottom (scrollbar.getHeight() + 4);
                thumbnail.drawChannels (g, thumbArea.reduced (2),
                                        visibleRange.getStart(), visibleRange.getEnd(), 1.0f);
            }
            else
            {
                g.setFont (14.0f);
                g.drawFittedText ("(No audio file selected)", getLocalBounds(), Justification::centred, 2);
            }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            scrollbar.setBounds (getLocalBounds().removeFromBottom (14).reduced (2));
        */
    }
    
    pub fn is_interested_in_file_drag(&mut self, files: &Vec<String>) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn files_dropped(
        &mut self, 
        files: &Vec<String>,
        x:     i32,
        y:     i32

    ) {
        
        todo!();
        /*
            lastFileDropped = Url (File (files[0]));
            sendChangeMessage();
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseDrag (e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (canMoveTransport())
                transportSource.setPosition (jmax (0.0, xToTime ((float) e.x)));
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            transportSource.start();
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        _0:    &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            if (thumbnail.getTotalLength() > 0.0)
            {
                auto newStart = visibleRange.getStart() - wheel.deltaX * (visibleRange.getLength()) / 10.0;
                newStart = jlimit (0.0, jmax (0.0, thumbnail.getTotalLength() - (visibleRange.getLength())), newStart);

                if (canMoveTransport())
                    setRange ({ newStart, newStart + visibleRange.getLength() });

                if (wheel.deltaY != 0.0f)
                    zoomSlider.setValue (zoomSlider.getValue() - wheel.deltaY);

                repaint();
            }
        */
    }
    
    pub fn time_tox(&self, time: f64) -> f32 {
        
        todo!();
        /*
            if (visibleRange.getLength() <= 0)
                return 0;

            return (float) getWidth() * (float) ((time - visibleRange.getStart()) / visibleRange.getLength());
        */
    }
    
    pub fn x_to_time(&self, x: f32) -> f64 {
        
        todo!();
        /*
            return (x / (float) getWidth()) * (visibleRange.getLength()) + visibleRange.getStart();
        */
    }
    
    pub fn can_move_transport(&self) -> bool {
        
        todo!();
        /*
            return ! (isFollowingTransport && transportSource.isPlaying());
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (canMoveTransport())
                updateCursorPosition();
            else
                setRange (visibleRange.movedToStartAt (transportSource.getCurrentPosition() - (visibleRange.getLength() / 2.0)));
        */
    }
    
    pub fn update_cursor_position(&mut self)  {
        
        todo!();
        /*
            currentPositionMarker.setVisible (transportSource.isPlaying() || isMouseButtonDown());

            currentPositionMarker.setRectangle (Rectangle<float> (timeToX (transportSource.getCurrentPosition()) - 0.75f, 0,
                                                                  1.5f, (float) (getHeight() - scrollbar.getHeight())));
        */
    }
}
