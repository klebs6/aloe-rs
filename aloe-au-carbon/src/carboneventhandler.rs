crate::ix!();

pub trait CarbonEventHandlerInterface {

    fn want_event_types(&mut self, 
        target:       EventTargetRef,
        in_num_types: u32,
        in_list:      *const EventTypeSpec);

    fn handle_event(&mut self, 
        in_handler_ref: EventHandlerCallRef,
        event:          EventRef) -> bool;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CarbonEventHandler.h]

pub struct CarbonEventHandler {
    handlers: CFMutableDictionaryRef,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CarbonEventHandler.cpp]

pub fn the_event_handler(
        in_handler_ref: EventHandlerCallRef,
        in_event:       EventRef,
        in_user_data:   *mut c_void) -> OSStatus {
    
    todo!();
        /*
            CarbonEventHandler *handler = (CarbonEventHandler *)inUserData;
        if (handler->HandleEvent(inHandlerRef, inEvent))
            return noErr;
        else return eventNotHandledErr;
        */
}

impl Drop for CarbonEventHandler {

    fn drop(&mut self) {
        todo!();
        /*
            if (mHandlers != NULL) {
            int count = static_cast<int>(CFDictionaryGetCount(mHandlers));
            EventHandlerRef *theHandlers = (EventHandlerRef*) malloc(count * sizeof(EventHandlerRef));
            CFDictionaryGetKeysAndValues(mHandlers, NULL, (const void **)theHandlers);

            for (int i = 0; i < count; i++)
                RemoveEventHandler(theHandlers[i]);
            CFDictionaryRemoveAllValues(mHandlers);
            CFRelease (mHandlers);
            free(theHandlers);
        }
        */
    }
}

impl Default for CarbonEventHandler {

    fn default() -> Self {
    
        todo!();
        /*
        : handlers(NULL),

        
        */
    }
}
    
impl CarbonEventHandler {

    pub fn want_event_types(&mut self, 
        target:       EventTargetRef,
        in_num_types: u32,
        in_list:      *const EventTypeSpec)  {
        
        todo!();
        /*
            if (mHandlers == NULL)
            mHandlers = CFDictionaryCreateMutable(NULL, 0, NULL, NULL);

        EventHandlerRef handler;

        if (CFDictionaryGetValueIfPresent (mHandlers, target, (const void **)&handler)) // if there is already a handler for the target, add the type
            verify_noerr(AddEventTypesToHandler(handler, inNumTypes, inList));
        else {
            verify_noerr(InstallEventHandler(target, TheEventHandler, inNumTypes, inList, this, &handler));
            CFDictionaryAddValue(mHandlers, target, handler);
        }
        */
    }
}
