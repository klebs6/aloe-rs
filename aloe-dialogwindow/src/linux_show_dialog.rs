crate::ix!();

pub enum Async {
    yes,
    no,
}

//#[cfg(target_os="linux")]
pub fn show_dialog(
    options:  &MessageBoxOptions,
    callback: *mut dyn ModalComponentManagerCallback,
    async_:    Async) -> i32 {

    todo!();
        /*
            const auto dummyCallback = [] (int) {};

        switch (options.getNumButtons())
        {
            case 2:
            {
                if (async == Async::yes && callback == nullptr)
                    callback = ModalCallbackFunction::create (dummyCallback);

                return AlertWindow::showOkCancelBox (options.getIconType(),
                                                     options.getTitle(),
                                                     options.getMessage(),
                                                     options.getButtonText (0),
                                                     options.getButtonText (1),
                                                     options.getAssociatedComponent(),
                                                     callback) ? 1 : 0;
            }

            case 3:
            {
                if (async == Async::yes && callback == nullptr)
                    callback = ModalCallbackFunction::create (dummyCallback);

                return AlertWindow::showYesNoCancelBox (options.getIconType(),
                                                        options.getTitle(),
                                                        options.getMessage(),
                                                        options.getButtonText (0),
                                                        options.getButtonText (1),
                                                        options.getButtonText (2),
                                                        options.getAssociatedComponent(),
                                                        callback);
            }

            case 1:
            default:
                break;
        }

       #if ALOE_MODAL_LOOPS_PERMITTED
        if (async == Async::no)
        {
            AlertWindow::showMessageBox (options.getIconType(),
                                         options.getTitle(),
                                         options.getMessage(),
                                         options.getButtonText (0),
                                         options.getAssociatedComponent());
        }
        else
       #endif
        {
            AlertWindow::showMessageBoxAsync (options.getIconType(),
                                              options.getTitle(),
                                              options.getMessage(),
                                              options.getButtonText (0),
                                              options.getAssociatedComponent(),
                                              callback);
        }

        return 0;
        */

}

