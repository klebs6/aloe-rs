crate::ix!();

#[no_copy]
#[leak_detector]
pub struct VideoComponentPimpl<'a> {
    base:                   Base<'a>,
    current_file:           File,
    currenturl:             Url,
    owner:                  &'a mut VideoComponent<'a>,
    player_controller:      PlayerController<'a>,
    load_finished_callback: fn(_0: &Url, _1: Result<(),()>) -> (),
    play_speed_mult:        f64, // default = 1.0
}

impl<'a> Drop for VideoComponentPimpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            close();
                setView (nil);
        */
    }
}

impl<'a> VideoComponentPimpl<'a> {
    
    pub fn new(
        owner_to_use:                     &mut VideoComponent,
        use_native_controls_if_available: bool) -> Self {
    
        todo!();
        /*

            : owner (ownerToUse),
                  playerController (*this, useNativeControlsIfAvailable)
                setVisible (true);

                auto* view = playerController.getView();
                setView (view);

               #if ALOE_MAC
                [view setNextResponder: [view superview]];
                [view setWantsLayer: YES];
               #endif
        */
    }
    
    pub fn load_file(&mut self, file: &File) -> Result<(),()> {
        
        todo!();
        /*
            auto r = load (createNSURLFromFile (file));

                if (r.wasOk())
                    currentFile = file;

                return r;
        */
    }
    
    pub fn load_url(&mut self, url: &Url) -> Result<(),()> {
        
        todo!();
        /*
            auto r = load ([NSURL URLWithString: aloeStringToNS (url.toString (true))]);

                if (r.wasOk())
                    currentURL = url;

                return r;
        */
    }
    
    pub fn load_url_ptr(&mut self, url: *mut NSURL) -> Result<(),()> {
        
        todo!();
        /*
            if (url != nil)
                {
                    close();
                    return playerController.load (url);
                }

                return Result::fail ("Couldn't open movie");
        */
    }
    
    pub fn load_async(&mut self, 
        url:      &Url,
        callback: fn(_0: &Url, _1: Result<(),()>) -> ())  {
        
        todo!();
        /*
            if (url.isEmpty())
                {
                    jassertfalse;
                    return;
                }

                currentURL = url;

                jassert (callback != nullptr);

                loadFinishedCallback = std::move (callback);

                playerController.loadAsync (url);
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            stop();
                playerController.close();
                currentFile = File();
                currentURL = {};
        */
    }
    
    pub fn is_open(&self) -> bool {
        
        todo!();
        /*
            return playerController.getPlayer() != nil;
        */
    }
    
    pub fn is_playing(&self) -> bool {
        
        todo!();
        /*
            return getSpeed() != 0;
        */
    }
    
    pub fn play(&mut self)  {
        
        todo!();
        /*
            [playerController.getPlayer() play]; setSpeed (playSpeedMult);
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            [playerController.getPlayer() pause];
        */
    }
    
    pub fn set_position(&mut self, new_position: f64)  {
        
        todo!();
        /*
            if (auto* p = playerController.getPlayer())
                {
                    CMTime t = { (CMTimeValue) (100000.0 * newPosition),
                                 (CMTimeScale) 100000, kCMTimeFlags_Valid, {} };

                    [p seekToTime: t
                  toleranceBefore: kCMTimeZero
                   toleranceAfter: kCMTimeZero];
                }
        */
    }
    
    pub fn get_position(&self) -> f64 {
        
        todo!();
        /*
            if (auto* p = playerController.getPlayer())
                    return toSeconds ([p currentTime]);

                return 0.0;
        */
    }
    
    pub fn set_speed(&mut self, new_speed: f64)  {
        
        todo!();
        /*
            playSpeedMult = newSpeed;

                // Calling non 0.0 speed on a paused player would start it...
                if (isPlaying())
                    [playerController.getPlayer() setRate: (float) playSpeedMult];
        */
    }
    
    pub fn get_speed(&self) -> f64 {
        
        todo!();
        /*
            if (auto* p = playerController.getPlayer())
                    return [p rate];

                return 0.0;
        */
    }
    
    pub fn get_native_size(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            if (auto* p = playerController.getPlayer())
                {
                    auto s = [[p currentItem] presentationSize];
                    return { (int) s.width, (int) s.height };
                }

                return {};
        */
    }
    
    pub fn get_duration(&self) -> f64 {
        
        todo!();
        /*
            if (auto* p = playerController.getPlayer())
                    return toSeconds ([[p currentItem] duration]);

                return 0.0;
        */
    }
    
    pub fn set_volume(&mut self, new_volume: f32)  {
        
        todo!();
        /*
            [playerController.getPlayer() setVolume: newVolume];
        */
    }
    
    pub fn get_volume(&self) -> f32 {
        
        todo!();
        /*
            if (auto* p = playerController.getPlayer())
                    return [p volume];

                return 0.0f;
        */
    }
    
    pub fn to_seconds(t: &CMTime) -> f64 {
        
        todo!();
        /*
            return t.timescale != 0 ? (t.value / (double) t.timescale) : 0.0;
        */
    }
    
    pub fn player_preparation_finished(&mut self, 
        url: &Url,
        r:   Result<(),()>)  {
        
        todo!();
        /*
            owner.resized();

                loadFinishedCallback (url, r);
                loadFinishedCallback = nullptr;
        */
    }
    
    pub fn error_occurred(&mut self, error_message: &String)  {
        
        todo!();
        /*
            if (owner.onErrorOccurred != nullptr)
                    owner.onErrorOccurred (errorMessage);
        */
    }
    
    pub fn playback_started(&mut self)  {
        
        todo!();
        /*
            if (owner.onPlaybackStarted != nullptr)
                    owner.onPlaybackStarted();
        */
    }
    
    pub fn playback_stopped(&mut self)  {
        
        todo!();
        /*
            if (owner.onPlaybackStopped != nullptr)
                    owner.onPlaybackStopped();
        */
    }
    
    pub fn playback_reached_end_time(&mut self)  {
        
        todo!();
        /*
            stop();
                setPosition (0.0);
        */
    }
}
