crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUCarbonViewBase.h]

pub const K_DEFAULT_NOTIFICATION_INTERVAL: f32 = 0.100;

pub type AUCarbonViewBaseControlList = Vec<*mut AUCarbonViewControl>;

pub struct AUCarbonViewBase {
    base:                     ComponentBase,
    base2:                    CarbonEventHandler,

    /**
      | the AU we're controlling
      |
      */
    edit_audio_unit:          AudioUnit,

    parameter_listener:       AUEventListenerRef,

    #[cfg(not(__LP64__))]
    event_listener:           AudioUnitCarbonViewEventListener,

    event_listener_user_data: *mut c_void,
    control_list:             AUCarbonViewBaseControlList,
    timer_ref:                EventLoopTimerRef,
    timerupp:                 EventLoopTimerUPP,
    carbon_window:            WindowRef,

    /**
      | user pane, contains all other controls
      |
      */
    carbon_pane:              ControlRef,

    /**
      | largest width and height of child controls
      |
      */
    bottom_right:             Point,

    xoffset:                  f32,
    yoffset:                  f32,
    composit_window:          bool,

    /**
      | needed for scrolling
      |
      */
    current_scroll_point:     HIPoint,
}

impl AUCarbonViewBaseInterface for AUCarbonViewBase {

    fn create_carbon_view(&mut self, 
        in_audio_unit:      AudioUnit,
        in_window:          WindowRef,
        in_parent_control:  ControlRef,
        in_location:        &Float32Point,
        in_size:            &Float32Point,
        out_parent_control: &mut ControlRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    fn createui(&mut self, 
        in_xoffset: Float32,
        in_yoffset: Float32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    fn handle_event(&mut self, 
        in_handler_ref: EventHandlerCallRef,
        event:          EventRef) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    fn respond_to_event_timer(&mut self, in_timer: EventLoopTimerRef)  {
        
        todo!();
        /*
        
        */
    }
}

impl AUCarbonViewBase {

    pub fn new(
        in_instance:              AudioUnitCarbonView,
        in_notification_interval: Option<Float32>

    ) -> Self {

        /* in seconds */
        let in_notification_interval: Float32 =
            in_notification_interval.unwrap_or(K_DEFAULT_NOTIFICATION_INTERVAL);

        todo!();
        /*
        
        */
    }
    
    pub fn get_edit_audio_unit(&self) -> AudioUnit {
        
        todo!();
        /*
            return mEditAudioUnit;
        */
    }
    
    pub fn add_carbon_control(&mut self, 
        ty:      AUCarbonViewControlType,
        param:   &CAAUParameter,
        control: ControlRef)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_carbon_window(&mut self) -> WindowRef {
        
        todo!();
        /*
            return mCarbonWindow;
        */
    }
    
    pub fn get_carbon_pane(&mut self) -> ControlRef {
        
        todo!();
        /*
            return mCarbonPane;
        */
    }
    
    pub fn embed_control(&mut self, ctl: ControlRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    pub fn tell_listener(&mut self, 
        auvp:  &CAAUParameter,
        event: AudioUnitCarbonViewEventID,
        evpar: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | pass in true if wanting an update to the
      | view and you're calling this from a thread
      | that is safe to do UI in.
      |
      | If you don't know, pass in false!
      */
    pub fn update(&mut self, in_ui_thread: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_xoffset(&mut self) -> Float32 {
        
        todo!();
        /*
            return mXOffset;
        */
    }
    
    pub fn get_yoffset(&mut self) -> Float32 {
        
        todo!();
        /*
            return mYOffset;
        */
    }
    
    pub fn clear_controls(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_composit_window(&self) -> bool {
        
        todo!();
        /*
            return mCompositWindow;
        */
    }

    #[cfg(not(__LP64__))]
    pub fn set_event_listener(&mut self, 
        listener:  AudioUnitCarbonViewEventListener,
        user_data: *mut c_void)  {
        
        todo!();
        /*
            mEventListener = listener;
                                        mEventListenerUserData = userData;
        */
    }
    
    pub fn add_control(&mut self, control: *mut AUCarbonViewControl)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn remove_control(&mut self, control: *mut AUCarbonViewControl)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn create_event_loop_timer(&mut self, 
        in_delay:    f32,
        in_interval: f32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    pub fn parameter_listener(
        in_callback_ref_con: *mut c_void,
        in_object:           *mut c_void,
        in_event:            *const AudioUnitEvent,
        in_event_host_time:  u64,
        in_parameter_value:  f32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn the_timer_proc(
        in_timer:     EventLoopTimerRef,
        in_user_data: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }

    pub fn component_entry_dispatch(&mut self, 
        p:    *mut ComponentParameters,
        this: *mut AUCarbonViewBase) -> OSStatus {
        
        todo!();
        /*
            if (This == NULL) return paramErr;

        OSStatus result = noErr;

        switch (p->what) {
        case kAudioUnitCarbonViewCreateSelect:
            {
                AudioUnitCarbonViewCreateGluePB *pb = (AudioUnitCarbonViewCreateGluePB *)p;
                CheckNull(pb->inAudioUnit);
                CheckNull(pb->inWindow);
                CheckNull(pb->inParentControl);
                CheckNull(pb->inSize);
                CheckNull(pb->inLocation);
                CheckNull(pb->outControl);
                result = This->CreateCarbonView(pb->inAudioUnit, pb->inWindow, pb->inParentControl,
                        *pb->inLocation, *pb->inSize, *pb->outControl);
            }
            break;
    #if !__LP64__
        case kAudioUnitCarbonViewSetEventListenerSelect:
            {
                AudioUnitCarbonViewSetEventListenerGluePB *pb = (AudioUnitCarbonViewSetEventListenerGluePB *)p;
                This->SetEventListener(pb->inCallback, pb->inUserData);
            }
            break;
    #endif

        default:
            result = ComponentBase::ComponentEntryDispatch(p, This);
            break;
        }
        return result;
        */
    }
}
