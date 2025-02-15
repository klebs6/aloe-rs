crate::ix!();

#[cfg(target_os="macos")]
pub struct PlayerController<'a> {
    base:         PlayerControllerBase<'a,Self>,
    view:         *mut NSView, // default = nil
    player_layer: *mut AVPlayerLayer, // default = nil

    /**
       32-bit builds don't have AVPlayerView
      */
    #[cfg(not(ALOE_32BIT))]
    player_view:  *mut AVPlayerView, // default = nil
}

#[cfg(target_os="macos")]
impl<'a> Drop for PlayerController<'a> {
    fn drop(&mut self) {
        todo!();
        /*
            #if ALOE_32BIT
                [view release];
                [playerLayer release];
               #else
                [playerView release];
               #endif
        */
    }
}

#[cfg(target_os="macos")]
impl<'a> PlayerController<'a> {

    pub fn new(
        owner_to_use:                     &mut VideoComponentImpl,
        use_native_controls_if_available: bool) -> Self {
    
        todo!();
        /*
        : player_controller_base(ownerToUse, useNativeControlsIfAvailable),

            #if ALOE_32BIT
                // 32-bit builds don't have AVPlayerView, so need to use a layer
                useNativeControls = false;
               #endif

                if (useNativeControls)
                {
                   #if ! ALOE_32BIT
                    playerView = [[AVPlayerView alloc] init];
                   #endif
                }
                else
                {
                    view = [[NSView alloc] init];
                    playerLayer = [[AVPlayerLayer alloc] init];
                    [view setLayer: playerLayer];
                }
        */
    }
    
    pub fn get_view(&mut self) -> *mut NSView {
        
        todo!();
        /*
            #if ! ALOE_32BIT
                if (useNativeControls)
                    return playerView;
               #endif

                return view;
        */
    }
    
    pub fn load(&mut self, url: *mut NSURL) -> Result<(),()> {
        
        todo!();
        /*
            if (auto player = [AVPlayer playerWithURL: url])
                {
                    setPlayer (player);
                    return Result::ok();
                }

                return Result::fail ("Couldn't open movie");
        */
    }
    
    pub fn load_async(&mut self, url: Url)  {
        
        todo!();
        /*
            playerAsyncInitialiser.loadAsync (url);
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            setPlayer (nil);
        */
    }
    
    pub fn set_player(&mut self, player: *mut AVPlayer)  {
        
        todo!();
        /*
            #if ! ALOE_32BIT
                if (useNativeControls)
                    [playerView setPlayer: player];
                else
               #endif
                    [playerLayer setPlayer: player];

                if (player != nil)
                {
                    attachPlayerStatusObserver();
                    attachPlaybackObserver();
                }
                else
                {
                    detachPlayerStatusObserver();
                    detachPlaybackObserver();
                }
        */
    }
    
    pub fn get_player(&self) -> *mut AVPlayer {
        
        todo!();
        /*
            #if ! ALOE_32BIT
                if (useNativeControls)
                    return [playerView player];
               #endif

                return [playerLayer player];
        */
    }
}
