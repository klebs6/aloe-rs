crate::ix!();

///--------------------------------
pub struct AUPropertyControl {
    base:    CarbonEventHandler,
    control: ControlRef,
    view:    *mut AUCarbonViewBase,
    height:  i16,
}

impl AUPropertyControl {

    pub fn new(in_base: *mut AUCarbonViewBase) -> Self {
    
        todo!();
        /*
        : control(0),
        : view(inBase),
        : height(0),

        
        */
    }
    
    pub fn get_height(&mut self) -> i32 {
        
        todo!();
        /*
            return mHeight;
        */
    }
    
    pub fn handle_event(&mut self, 
        in_handler_ref: EventHandlerCallRef,
        event:          EventRef) -> bool {
        
        todo!();
        /*
            UInt32 eclass = GetEventClass(event);
        UInt32 ekind = GetEventKind(event);
        switch (eclass) {
        case kEventClassControl:
            switch (ekind) {
            case kEventControlValueFieldChanged:
                HandleControlChange();
                return true;    // handled
            }
        }

        return false;
        */
    }
    
    pub fn register_events(&mut self)  {
        
        todo!();
        /*
            #if !__LP64__
        EventTypeSpec events[] = {
            { kEventClassControl, kEventControlValueFieldChanged }  // N.B. OS X only
        };

        WantEventTypes(GetControlEventTarget(mControl), GetEventTypeCount(events), events);
    #endif
        */
    }
    
    pub fn embed_control(&mut self, the_control: ControlRef)  {
        
        todo!();
        /*
            mView->EmbedControl (theControl);
        */
    }
    
    pub fn get_carbon_window(&mut self) -> WindowRef {
        
        todo!();
        /*
            return mView->GetCarbonWindow();
        */
    }
}
