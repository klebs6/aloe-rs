crate::ix!();

#[weak_referenceable]
pub struct PlayerAsyncInitialiser<'a> {
    owner:                                   &'a mut PlayerControllerBase<'a,Self>,
    asset:                                   Box<AVURLAsset,NSObjectDeleter>,
    asset_keys:                              Box<NSArray<*mut NSString>,NSObjectDeleter>,
    player_item:                             Box<AVPlayerItem,NSObjectDeleter>,
    player_item_preparation_status_observer: Box<NSObject,NSObjectDeleter>,
    player:                                  Box<AVPlayer,NSObjectDeleter>,
}

impl<'a> Drop for PlayerAsyncInitialiser<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            detachPreparationStatusObserver();
        */
    }
}

impl<'a> PlayerAsyncInitialiser<'a> {

    pub fn new(owner_to_use: &mut PlayerControllerBase<'a,Self>) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                  assetKeys ([[NSArray alloc] initWithObjects: nsStringLiteral ("duration"), nsStringLiteral ("tracks"),
                                                               nsStringLiteral ("playable"), nil])
                static AloePlayerItemPreparationStatusObserverClass cls;
                playerItemPreparationStatusObserver.reset ([cls.createInstance() init]);
                AloePlayerItemPreparationStatusObserverClass::setOwner (playerItemPreparationStatusObserver.get(), this);
        */
    }
    
    pub fn load_async(&mut self, url: Url)  {
        
        todo!();
        /*
            auto nsUrl = [NSURL URLWithString: aloeStringToNS (url.toString (true))];
                asset.reset ([[AVURLAsset alloc] initWithURL: nsUrl options: nil]);

                [asset.get() loadValuesAsynchronouslyForKeys: assetKeys.get()
                                           completionHandler: ^() { checkAllKeysReadyFor (asset.get(), url); }];
        */
    }
    
    pub fn check_all_keys_ready_for(&mut self, 
        asset_to_check: *mut AVAsset,
        url:            &Url)  {
        
        todo!();
        /*
            NSError* error = nil;

                int successCount = 0;

                for (NSString* key : assetKeys.get())
                {
                    switch ([assetToCheck statusOfValueForKey: key error: &error])
                    {
                        case AVKeyValueStatusLoaded:
                        {
                            ++successCount;
                            break;
                        }
                        case AVKeyValueStatusCancelled:
                        {
                            notifyOwnerPreparationFinished (url, Result::fail ("Loading cancelled"), nullptr);
                            return;
                        }
                        case AVKeyValueStatusFailed:
                        {
                            auto errorMessage = error != nil ? nsStringToAloe (error.localizedDescription) : String();
                            notifyOwnerPreparationFinished (url, Result::fail (errorMessage), nullptr);
                            return;
                        }

                        case AVKeyValueStatusUnknown:
                        case AVKeyValueStatusLoading:
                        default:
                            break;
                    }
                }

                jassert (successCount == (int) [assetKeys.get() count]);
                preparePlayerItem();
        */
    }
    
    pub fn prepare_player_item(&mut self)  {
        
        todo!();
        /*
            playerItem.reset ([[AVPlayerItem alloc] initWithAsset: asset.get()]);

                attachPreparationStatusObserver();

                player.reset ([[AVPlayer alloc] initWithPlayerItem: playerItem.get()]);
        */
    }
    
    pub fn attach_preparation_status_observer(&mut self)  {
        
        todo!();
        /*
            [playerItem.get() addObserver: playerItemPreparationStatusObserver.get()
                                   forKeyPath: nsStringLiteral ("status")
                                      options: NSKeyValueObservingOptionOld | NSKeyValueObservingOptionNew
                                      context: this];
        */
    }
    
    pub fn detach_preparation_status_observer(&mut self)  {
        
        todo!();
        /*
            if (playerItem != nullptr && playerItemPreparationStatusObserver != nullptr)
                {
                    [playerItem.get() removeObserver: playerItemPreparationStatusObserver.get()
                                          forKeyPath: nsStringLiteral ("status")
                                             context: this];
                }
        */
    }
    
    pub fn notify_owner_preparation_finished(&mut self, 
        url:             &Url,
        r:               Result<(),()>,
        prepared_player: *mut AVPlayer)  {
        
        todo!();
        /*
            MessageManager::callAsync ([url, preparedPlayer, r,
                                            safeThis = WeakReference<PlayerAsyncInitialiser> { this }]() mutable
                {
                    if (safeThis != nullptr)
                        safeThis->owner.playerPreparationFinished (url, r, preparedPlayer);
                });
        */
    }
}
