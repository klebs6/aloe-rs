crate::ix!();

#[weak_referenceable]
pub struct PlayerControllerBase<'a, Derived> {
    owner:                                &'a mut VideoComponentImpl<'a>,
    use_native_controls:                  bool,
    player_async_initialiser:             PlayerAsyncInitialiser<'a>,
    player_status_observer:               Box<NSObject,NSObjectDeleter>,
    player_item_playback_status_observer: Box<NSObject,NSObjectDeleter>,
    _0:                                   PhantomData<Derived>,
}

impl<'a, Derived> Drop for PlayerControllerBase<'a, Derived> {

    fn drop(&mut self) {
        todo!();
        /*
            detachPlayerStatusObserver();
                detachPlaybackObserver();
        */
    }
}

impl<'a, Derived> PlayerControllerBase<'a, Derived> {

    pub fn new(
        owner_to_use:                     &mut VideoComponentImpl,
        use_native_controls_if_available: bool) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                  useNativeControls (useNativeControlsIfAvailable),
                  playerAsyncInitialiser (*this)

                static AloePlayerStatusObserverClass playerObserverClass;
                playerStatusObserver.reset ([playerObserverClass.createInstance() init]);
                AloePlayerStatusObserverClass::setOwner (playerStatusObserver.get(), this);

                static AloePlayerItemPlaybackStatusObserverClass itemObserverClass;
                playerItemPlaybackStatusObserver.reset ([itemObserverClass.createInstance() init]);
                AloePlayerItemPlaybackStatusObserverClass::setOwner (playerItemPlaybackStatusObserver.get(), this);
        */
    }
    
    pub fn attach_player_status_observer(&mut self)  {
        
        todo!();
        /*
            [crtp().getPlayer() addObserver: playerStatusObserver.get()
                                     forKeyPath: nsStringLiteral ("rate")
                                        options: NSKeyValueObservingOptionOld | NSKeyValueObservingOptionNew
                                        context: this];

                [crtp().getPlayer() addObserver: playerStatusObserver.get()
                                     forKeyPath: nsStringLiteral ("status")
                                        options: NSKeyValueObservingOptionNew
                                        context: this];
        */
    }
    
    pub fn detach_player_status_observer(&mut self)  {
        
        todo!();
        /*
            if (crtp().getPlayer() != nullptr && playerStatusObserver != nullptr)
                {
                        [crtp().getPlayer() removeObserver: playerStatusObserver.get()
                                                forKeyPath: nsStringLiteral ("rate")
                                                   context: this];

                        [crtp().getPlayer() removeObserver: playerStatusObserver.get()
                                                forKeyPath: nsStringLiteral ("status")
                                                   context: this];
                }
        */
    }
    
    pub fn attach_playback_observer(&mut self)  {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
                [[NSNotificationCenter defaultCenter] addObserver: playerItemPlaybackStatusObserver.get()
                                                         selector: @selector (processNotification:)
                                                             name: AVPlayerItemDidPlayToEndTimeNotification
                                                           object: [crtp().getPlayer() currentItem]];
                ALOE_END_IGNORE_WARNINGS_GCC_LIKE
        */
    }
    
    pub fn detach_playback_observer(&mut self)  {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
                [[NSNotificationCenter defaultCenter] removeObserver: playerItemPlaybackStatusObserver.get()];
                ALOE_END_IGNORE_WARNINGS_GCC_LIKE
        */
    }
    
    pub fn crtp(&mut self) -> &mut Derived {
        
        todo!();
        /*
            return static_cast<Derived&> (*this);
        */
    }
    
    pub fn player_preparation_finished(
        &mut self, 
        url:             &Url,
        r:               Result<(),()>,
        prepared_player: *mut AVPlayer
    ) {
        
        todo!();
        /*
            if (preparedPlayer != nil)
                    crtp().setPlayer (preparedPlayer);

                owner.playerPreparationFinished (url, r);
        */
    }
    
    pub fn playback_reached_end_time(&mut self)  {
        
        todo!();
        /*
            MessageManager::callAsync ([safeThis = WeakReference<PlayerControllerBase> { this }]() mutable
                                           {
                                               if (safeThis != nullptr)
                                                   safeThis->owner.playbackReachedEndTime();
                                           });
        */
    }
    
    pub fn error_occurred(&mut self)  {
        
        todo!();
        /*
            auto errorMessage = (crtp().getPlayer() != nil && crtp().getPlayer().error != nil)
                                  ? nsStringToAloe (crtp().getPlayer().error.localizedDescription)
                                  : String();

                owner.errorOccurred (errorMessage);
        */
    }
    
    pub fn playback_started(&mut self)  {
        
        todo!();
        /*
            owner.playbackStarted();
        */
    }
    
    pub fn playback_stopped(&mut self)  {
        
        todo!();
        /*
            owner.playbackStopped();
        */
    }
}
