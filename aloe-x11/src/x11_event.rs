crate::ix!();

pub fn get_event_time(t: Time) -> i64 {
    
    todo!();
        /*
            static int64 eventTimeOffset = 0x12345678;
        auto thisMessageTime = (int64) t;

        if (eventTimeOffset == 0x12345678)
            eventTimeOffset = Time::currentTimeMillis() - thisMessageTime;

        return eventTimeOffset + thisMessageTime;
        */
}

pub fn get_event_time_for_type<EventType>(t: &EventType) -> i64 {

    todo!();
        /*
            return getEventTime (t.time);
        */
}

pub fn get_all_events_mask(ignores_mouse_clicks: bool) -> i32 {
    
    todo!();
        /*
            return NoEventMask | KeyPressMask | KeyReleaseMask
                 | EnterWindowMask | LeaveWindowMask | PointerMotionMask | KeymapStateMask
                 | ExposureMask | StructureNotifyMask | FocusChangeMask | PropertyChangeMask
                 | (ignoresMouseClicks ? 0 : (ButtonPressMask | ButtonReleaseMask));
        */
}
