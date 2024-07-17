crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/playback/aloe_VideoComponent.h]

/**
  | A component that can play a movie.
  | 
  | Use the load() method to open a video
  | once you've added this component to
  | a parent (or put it on the desktop).
  | 
  | @tags{Video}
  |
  */
#[no_copy]
#[leak_detector]
pub struct VideoComponent<'a> {
    base:                           Component<'a>,
    base2:                          Timer,

    /**
      | Set this callback to be notified whenever
      | OS global media volume changes.
      | 
      | Currently used on Android only.
      |
      */
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    on_global_media_volume_changed: fn() -> (),

    /**
      | Set this callback to be notified whenever
      | the playback starts.
      |
      */
    on_playback_started: fn() -> (),

    /**
      | Set this callback to be notified whenever
      | the playback stops.
      |
      */
    on_playback_stopped: fn() -> (),

    /**
      | Set this callback to be notified whenever
      | an error occurs. Upon error, you may
      | need to load the video again.
      |
      */
    on_error_occurred:   fn(error: &String) -> (),

    pimpl:               Box<dyn VideoComponentImplInterface>,
}

pub trait VideoComponentImplInterface { }

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/playback/aloe_VideoComponent.cpp]
#[cfg(not(any(target_os="linux",target_os="bsd")))]
impl<'a> Drop for VideoComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            pimpl.reset();
        */
    }
}

#[cfg(not(any(target_os="linux",target_os="bsd")))]
impl<'a> VideoComponent<'a> {
    
    /**
      | Creates an empty VideoComponent.
      | 
      | Use the loadAsync() or load() method
      | to open a video once you've added this
      | component to a parent (or put it on the
      | desktop).
      | 
      | If useNativeControlsIfAvailable
      | is enabled and a target OS has a video
      | view with dedicated controls for transport
      | etc, that view will be used. In opposite
      | case a bare video view without any controls
      | will be presented, allowing you to tailor
      | your own UI. Currently this flag is used
      | on iOS and 64bit macOS.
      | 
      | Android, Windows and 32bit macOS will
      | always use plain video views without
      | dedicated controls.
      |
      */
    pub fn new(use_native_controls_if_available: bool) -> Self {
    
        todo!();
        /*
        : pimpl (new Pimpl (*this, useNativeControlsIfAvailable))
        addAndMakeVisible (pimpl.get());
        */
    }
    
    /**
      | Tries to load a video from a local file.
      | 
      | This function is supported on macOS
      | and Windows. For iOS and Android, use
      | loadAsync() instead.
      | 
      | 
      | -----------
      | @return
      | 
      | an error if the file failed to be loaded
      | correctly
      | 
      | @see loadAsync
      |
      */
    pub fn load_file(&mut self, file: &File) -> Result<(),()> {
        
        todo!();
        /*
            return loadInternal (file, false);
        */
    }
    
    /**
      | Tries to load a video from a Url.
      | 
      | This function is supported on macOS
      | and Windows. For iOS and Android, use
      | loadAsync() instead.
      | 
      | 
      | -----------
      | @return
      | 
      | an error if the file failed to be loaded
      | correctly
      | 
      | @see loadAsync
      |
      */
    pub fn load_url(&mut self, url: &Url) -> Result<(),()> {
        
        todo!();
        /*
            return loadInternal (url, false);
        */
    }
    
    /**
      | Tries to load a video from a Url asynchronously.
      | When finished, invokes the callback
      | supplied to the function on the message
      | thread.
      | 
      | This is the preferred way of loading
      | content, since it works not only on macOS
      | and Windows, but also on iOS and Android.
      | On Windows, it will internally call
      | load().
      | 
      | @see load
      |
      */
    pub fn load_async(&mut self, 
        url:      &Url,
        callback: fn(_0: &Url, _1: Result<(),()>) -> ())  {
        
        todo!();
        /*
            if (callback == nullptr)
        {
            jassertfalse;
            return;
        }

       #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        pimpl->loadAsync (url, callback);
       #else
        auto result = loadInternal (url, true);
        callback (url, result);
       #endif
        */
    }
    
    /**
      | Closes the video and resets the component.
      |
      */
    pub fn close_video(&mut self)  {
        
        todo!();
        /*
            pimpl->close();
        // Closing on Android is async and resized() will be called internally by pimpl once
        // close operation finished.
       #if ! ALOE_ANDROID// TODO ALOE_IOS too?
        resized();
       #endif
        */
    }
    
    /**
      | Returns true if a video is currently
      | open.
      |
      */
    pub fn is_video_open(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isOpen();
        */
    }
    
    /**
      | Returns the last file that was loaded.
      | 
      | If nothing is open, or if it was a Url rather
      | than a file, this will return File().
      |
      */
    pub fn get_current_video_file(&self) -> File {
        
        todo!();
        /*
            return pimpl->currentFile;
        */
    }
    
    /**
      | Returns the last Url that was loaded.
      | 
      | If nothing is open, or if it was a file
      | rather than a Url, this will return Url().
      |
      */
    pub fn get_current_videourl(&self) -> Url {
        
        todo!();
        /*
            return pimpl->currentURL;
        */
    }
    
    /**
      | Returns the length of the video, in seconds.
      |
      */
    pub fn get_video_duration(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->getDuration();
        */
    }
    
    /**
      | Returns the video's natural size, in
      | pixels.
      | 
      | If no video is loaded, an empty rectangle
      | will be returned.
      |
      */
    pub fn get_video_native_size(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return pimpl->getNativeSize();
        */
    }
    
    /**
      | Starts the video playing.
      |
      */
    pub fn play(&mut self)  {
        
        todo!();
        /*
            pimpl->play();
        */
    }
    
    /**
      | Stops the video playing.
      |
      */
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            pimpl->stop();
        */
    }
    
    /**
      | Returns true if the video is currently
      | playing.
      |
      */
    pub fn is_playing(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isPlaying();
        */
    }
    
    /**
      | Sets the video's position to a given
      | time.
      |
      */
    pub fn set_play_position(&mut self, new_pos: f64)  {
        
        todo!();
        /*
            pimpl->setPosition (newPos);
        */
    }
    
    /**
      | Returns the current play position of
      | the video.
      |
      */
    pub fn get_play_position(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->getPosition();
        */
    }
    
    /**
      | Changes the video playback rate.
      | 
      | A value of 1.0 is normal speed, greater
      | values will play faster, smaller values
      | play more slowly.
      |
      */
    pub fn set_play_speed(&mut self, new_speed: f64)  {
        
        todo!();
        /*
            pimpl->setSpeed (newSpeed);
        */
    }
    
    /**
      | Returns the current play speed of the
      | video.
      |
      */
    pub fn get_play_speed(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->getSpeed();
        */
    }
    
    /**
      | Changes the video's playback volume.
      | 
      | -----------
      | @param newVolume
      | 
      | the volume in the range 0 (silent) to
      | 1.0 (full)
      |
      */
    pub fn set_audio_volume(&mut self, new_volume: f32)  {
        
        todo!();
        /*
            pimpl->setVolume (newVolume);
        */
    }
    
    /**
      | Returns the video's playback volume.
      | 
      | 
      | -----------
      | @return
      | 
      | the volume in the range 0 (silent) to
      | 1.0 (full)
      |
      */
    pub fn get_audio_volume(&self) -> f32 {
        
        todo!();
        /*
            return pimpl->getVolume();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds();

        if (isVideoOpen() && ! r.isEmpty())
        {
            auto nativeSize = getVideoNativeSize();

            if (nativeSize.isEmpty())
            {
                // if we've just opened the file and are still waiting for it to
                // figure out the size, start our timer..
                if (! isTimerRunning())
                    startTimer (50);
            }
            else
            {
                r = RectanglePlacement (RectanglePlacement::centred).appliedTo (nativeSize, r);
                stopTimer();
            }
        }
        else
        {
            stopTimer();
        }

        pimpl->setBounds (r);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            resized();
        */
    }
    
    pub fn load_internal<FileOrURL>(&mut self, 
        file_or_url: &FileOrURL,
        load_async:  bool) -> Result<(),()> {
    
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS
        ignoreUnused (fileOrUrl, loadAsync);

        // You need to use loadAsync on Android & iOS.
        jassertfalse;
        return Result::fail ("load() is not supported on this platform. Use loadAsync() instead.");
       #else
        auto result = pimpl->load (fileOrUrl);

        if (loadAsync)
            startTimer (50);
        else
           resized();

        return result;
       #endif
        */
    }
}
