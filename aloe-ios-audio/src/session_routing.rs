crate::ix!();

pub fn get_routing_change_reason(reason: AVAudioSessionRouteChangeReason) -> *const u8 {
    
    todo!();
    /*
        switch (reason)
        {
            case AVAudioSessionRouteChangeReasonNewDeviceAvailable:         return "New device available";
            case AVAudioSessionRouteChangeReasonOldDeviceUnavailable:       return "Old device unavailable";
            case AVAudioSessionRouteChangeReasonCategoryChange:             return "Category change";
            case AVAudioSessionRouteChangeReasonOverride:                   return "Override";
            case AVAudioSessionRouteChangeReasonWakeFromSleep:              return "Wake from sleep";
            case AVAudioSessionRouteChangeReasonNoSuitableRouteForCategory: return "No suitable route for category";
            case AVAudioSessionRouteChangeReasonRouteConfigurationChange:   return "Route configuration change";
            case AVAudioSessionRouteChangeReasonUnknown:
            default:                                                        return "Unknown";
        }
    */
}

pub fn get_notification_value_for_key(
        notification: *mut NSNotification,
        key:          *mut NSString,
        value:        &mut NSUInteger) -> bool {
    
    todo!();
    /*
        if (notification != nil)
        {
            if (NSDictionary* userInfo = [notification userInfo])
            {
                if (NSNumber* number = [userInfo objectForKey: key])
                {
                    value = [number unsignedIntegerValue];
                    return true;
                }
            }
        }

        jassertfalse;
        return false;
    */
}

lazy_static!{
    /*
    @interface iOSAudioSessionNative  : NSObject
    {
    @private
        aloe::AudioSessionHolder* audioSessionHolder;
    };

    - (id) init: (aloe::AudioSessionHolder*) holder;
    - (void) dealloc;

    - (void) audioSessionChangedInterruptionType: (NSNotification*) notification;
    - (void) handleMediaServicesReset;
    - (void) handleMediaServicesLost;
    - (void) handleRouteChange: (NSNotification*) notification;
    @end

    @implementation iOSAudioSessionNative

    - (id) init: (aloe::AudioSessionHolder*) holder
    {
        self = [super init];

        if (self != nil)
        {
            audioSessionHolder = holder;

            auto session = [AVAudioSession sharedInstance];
            auto centre = [NSNotificationCenter defaultCenter];

            [centre addObserver: self
                       selector: @selector (audioSessionChangedInterruptionType:)
                           name: AVAudioSessionInterruptionNotification
                         object: session];

            [centre addObserver: self
                       selector: @selector (handleMediaServicesLost)
                           name: AVAudioSessionMediaServicesWereLostNotification
                         object: session];

            [centre addObserver: self
                       selector: @selector (handleMediaServicesReset)
                           name: AVAudioSessionMediaServicesWereResetNotification
                         object: session];

            [centre addObserver: self
                       selector: @selector (handleRouteChange:)
                           name: AVAudioSessionRouteChangeNotification
                         object: session];
        }
        else
        {
            jassertfalse;
        }

        return self;
    }

    - (void) dealloc
    {
        [[NSNotificationCenter defaultCenter] removeObserver: self];
        [super dealloc];
    }

    - (void) audioSessionChangedInterruptionType: (NSNotification*) notification
    {
        NSUInteger value;

        if (aloe::getNotificationValueForKey (notification, AVAudioSessionInterruptionTypeKey, value))
        {
            switch ((AVAudioSessionInterruptionType) value)
            {
                case AVAudioSessionInterruptionTypeBegan:
                    audioSessionHolder->handleStatusChange (false, "AVAudioSessionInterruptionTypeBegan");
                    break;

                case AVAudioSessionInterruptionTypeEnded:
                    audioSessionHolder->handleStatusChange (true, "AVAudioSessionInterruptionTypeEnded");
                    break;

                // No default so the code doesn't compile if this enum is extended.
            }
        }
    }

    - (void) handleMediaServicesReset
    {
        audioSessionHolder->handleStatusChange (true, "AVAudioSessionMediaServicesWereResetNotification");
    }

    - (void) handleMediaServicesLost
    {
        audioSessionHolder->handleStatusChange (false, "AVAudioSessionMediaServicesWereLostNotification");
    }

    - (void) handleRouteChange: (NSNotification*) notification
    {
        NSUInteger value;

        if (aloe::getNotificationValueForKey (notification, AVAudioSessionRouteChangeReasonKey, value))
            audioSessionHolder->handleRouteChange ((AVAudioSessionRouteChangeReason) value);
    }

    @end
    */
}


