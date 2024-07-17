crate::ix!();

pub struct AudioThumbnailComponent<'a> {
    base:                 Component<'a>,
    base3:                ChangeBroadcaster<'a>,
    base5:                Timer,
    audio_device_manager: &'a mut AudioDeviceManager<'a>,
    thumbnail_cache:      AudioThumbnailCache,
    thumbnail:            AudioThumbnail<'a>,
    transport_source:     *mut AudioTransportSource<'a>, // default = nullptr
    currenturl:           Url,
    current_position:     f64, // default = 0.0
}

impl<'a> FileDragAndDropTarget       for AudioThumbnailComponent<'a> {}
impl<'a> ShouldDrawDragImageWhenOver for AudioThumbnailComponent<'a> {}
impl<'a> ItemDragExit                for AudioThumbnailComponent<'a> {}
impl<'a> ItemDragMove                for AudioThumbnailComponent<'a> {}
impl<'a> ItemDragEnter               for AudioThumbnailComponent<'a> {}

impl<'a> ItemDropped for AudioThumbnailComponent<'a> {

    fn item_dropped(&mut self, _: &DragAndDropTargetSourceDetails<'_>) { 
        todo!();
    }
}

impl<'a> IsInterestedInDragSource for AudioThumbnailComponent<'a> {

    fn is_interested_in_drag_source(&mut self, _: &DragAndDropTargetSourceDetails<'_>) -> bool { 
        todo!();
    }
}

impl<'a> ChangeListener for AudioThumbnailComponent<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            repaint();
        */
    }
}

impl<'a> Drop for AudioThumbnailComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            thumbnail.removeChangeListener (this);
         */
    }
}

impl<'a> AudioThumbnailComponent<'a> {

    pub fn new(
        adm: &mut AudioDeviceManager,
        afm: &mut AudioFormatManager) -> Self {
    
        todo!();
        /*
        : audio_device_manager(adm),
        : thumbnail_cache(5),
        : thumbnail(128, afm, thumbnailCache),

            thumbnail.addChangeListener (this);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colour (0xff495358));

            g.setColour (Colours::white);

            if (thumbnail.getTotalLength() > 0.0)
            {
                thumbnail.drawChannels (g, getLocalBounds().reduced (2),
                                        0.0, thumbnail.getTotalLength(), 1.0f);

                g.setColour (Colours::black);
                g.fillRect (static_cast<FloatType> (currentPosition * getWidth()), 0.0f,
                            1.0f, static_cast<FloatType> (getHeight()));
            }
            else
            {
                g.drawFittedText ("No audio file loaded.\nDrop a file here or click the \"Load File...\" button.", getLocalBounds(),
                                  Justification::centred, 2);
            }
        */
    }
    
    pub fn is_interested_in_file_drag(&mut self, _0: &Vec<String>) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn files_dropped(&mut self, 
        files: &Vec<String>,
        _1:    i32,
        _2:    i32)  {
        
        todo!();
        /*
            loadURL (Url (File (files[0])), true);
        */
    }
    
    pub fn set_currenturl(&mut self, u: &Url)  {
        
        todo!();
        /*
            if (currentURL == u)
                return;

            loadURL (u);
        */
    }
    
    pub fn get_currenturl(&mut self) -> Url {
        
        todo!();
        /*
            return currentURL;
        */
    }
    
    pub fn set_transport_source(&mut self, new_source: *mut AudioTransportSource)  {
        
        todo!();
        /*
            transportSource = newSource;

            struct ResetCallback  : public CallbackMessage
            {
                ResetCallback (AudioThumbnailComponent& o) : owner (o) {}
                void messageCallback() override    { owner.reset(); }

                AudioThumbnailComponent& owner;
            };

            (new ResetCallback (*this))->post();
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            currentPosition = 0.0;
            repaint();

            if (transportSource == nullptr)
                stopTimer();
            else
                startTimerHz (25);
        */
    }
    
    pub fn loadurl(
        &mut self, 
        u:      &Url,
        notify: Option<bool>

    ) {

        let notify: bool = notify.unwrap_or(false);

        todo!();
        /*
            if (currentURL == u)
                return;

            currentURL = u;

            InputSource* inputSource = nullptr;

           #if ! ALOE_IOS
            if (u.isLocalFile())
            {
                inputSource = new FileInputSource (u.getLocalFile());
            }
            else
           #endif
            {
                if (inputSource == nullptr)
                    inputSource = new URLInputSource (u);
            }

            thumbnail.setSource (inputSource);

            if (notify)
                sendChangeMessage();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (transportSource != nullptr)
            {
                currentPosition = transportSource->getCurrentPosition() / thumbnail.getTotalLength();
                repaint();
            }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (transportSource != nullptr)
            {
                const ScopedLock sl (audioDeviceManager.getAudioCallbackLock());

                transportSource->setPosition ((jmax (static_cast<double> (e.x), 0.0) / getWidth())
                                                * thumbnail.getTotalLength());
            }
        */
    }
}
