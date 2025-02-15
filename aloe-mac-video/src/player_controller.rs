crate::ix!();

#[cfg(not(target_os="macos"))]
pub struct PlayerController {
    base:                   PlayerControllerBase<PlayerController>,
    player_view_controller: Box<AVPlayerViewController,NSObjectDeleter>,
    player_view:            Box<UIView,NSObjectDeleter>,
    player_layer:           Box<AVPlayerLayer,NSObjectDeleter>,
}

#[cfg(not(target_os="macos"))]
impl PlayerController {

    pub fn new(
        owner_to_use:                     &mut VideoComponentImpl,
        use_native_controls_if_available: bool) -> Self {
    
        todo!();
        /*
        : player_controller_base(ownerToUse, useNativeControlsIfAvailable),

            if (useNativeControls)
                {
                    playerViewController.reset ([[AVPlayerViewController alloc] init]);
                }
                else
                {
                    static AloeVideoViewerClass cls;
                    playerView.reset ([cls.createInstance() init]);

                    playerLayer.reset ([[AVPlayerLayer alloc] init]);
                    [playerView.get().layer addSublayer: playerLayer.get()];
                }
        */
    }
    
    pub fn get_view(&mut self) -> *mut UIView {
        
        todo!();
        /*
            if (useNativeControls)
                    return [playerViewController.get() view];

                // Should call getView() only once.
                jassert (playerView != nil);
                return playerView.release();
        */
    }
    
    pub fn load(&mut self, _0: *mut NSURL) -> Result {
        
        todo!();
        /*
            jassertfalse;
                return Result::fail ("Synchronous loading is not supported on iOS, use loadAsync()");
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
    
    pub fn get_player(&self) -> *mut AVPlayer {
        
        todo!();
        /*
            if (useNativeControls)
                    return [playerViewController.get() player];

                return [playerLayer.get() player];
        */
    }
    
    pub fn set_player(&mut self, player_to_use: *mut AVPlayer)  {
        
        todo!();
        /*
            if (useNativeControls)
                    [playerViewController.get() setPlayer: playerToUse];
                else
                    [playerLayer.get() setPlayer: playerToUse];

                attachPlayerStatusObserver();
                attachPlaybackObserver();
        */
    }
}
