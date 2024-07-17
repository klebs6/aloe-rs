crate::ix!();

pub fn show_maybe_async(
    options:     &MessageBoxOptions,
    callback_in: *mut dyn ModalComponentManagerCallback,
    map_fn:      AlertWindowMappingsMapFn

) -> i32 {
    
    todo!();
        /*
            const auto showAsync = (callbackIn != nullptr ? Async::yes
                                                      : Async::no);

        auto callback = AlertWindowMappings::getWrappedCallback (callbackIn, mapFn);

        if (LookAndFeel::getDefaultLookAndFeel().isUsingNativeAlertWindows())
        {
           #if ALOE_MODAL_LOOPS_PERMITTED
            if (showAsync == Async::no)
                return mapFn (NativeMessageBox::show (options));
           #endif

            NativeMessageBox::showAsync (options, callback.release());
            return false;
        }

        AlertWindowInfo info (options, std::move (callback), showAsync);
        return info.invoke();
        */
}

