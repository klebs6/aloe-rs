crate::ix!();

pub type AlertWindowMappingsMapFn = fn(_0: i32) -> i32;

#[inline] pub fn alert_window_mappings_no_mapping(button_index: i32) -> i32 {
    
    todo!();
        /*
            return buttonIndex;
        */
}

#[inline] pub fn alert_window_mappings_message_box(_0: i32) -> i32 {
    
    todo!();
        /*
            return 0;
        */
}

#[inline] pub fn alert_window_mappings_ok_cancel(button_index: i32) -> i32 {
    
    todo!();
        /*
            return buttonIndex == 0 ? 1 : 0;
        */
}

#[inline] pub fn alert_window_mappings_yes_no_cancel(button_index: i32) -> i32 {
    
    todo!();
        /*
            return buttonIndex == 2 ? 0 : buttonIndex + 1;
        */
}

pub fn alert_window_mappings_get_wrapped_callback(
    callback_in: *mut dyn ModalComponentManagerCallback,
    map_fn:      AlertWindowMappingsMapFn

) -> Box<dyn ModalComponentManagerCallback> {
    
    todo!();
        /*
            jassert (mapFn != nullptr);

            if (callbackIn == nullptr)
                return nullptr;

            auto wrappedCallback = [innerCallback = rawToUniquePtr (callbackIn), mapFn] (int buttonIndex)
            {
                innerCallback->modalStateFinished (mapFn (buttonIndex));
            };

            return rawToUniquePtr (ModalCallbackFunction::create (std::move (wrappedCallback)));
        */
}
