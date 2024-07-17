crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_NativeMessageBox.h]

pub struct NativeMessageBox(*mut c_void);

/**
  | This class contains some static methods
  | for showing native alert windows.
  | 
  | @tags{GUI}
  |
  */
impl NativeMessageBox {

    /**
      | Shows a dialog box that just has a message
      | and a single 'ok' button to close it.
      | 
      | The box is shown modally, and the method
      | will block until the user has clicked
      | its button (or pressed the escape or
      | return keys).
      | 
      | -----------
      | @param iconType
      | 
      | the type of icon to show.
      | ----------
      | @param title
      | 
      | the headline to show at the top of the
      | box.
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the title.
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_message_box(
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component<'a>)  {
        let associated_component: *mut Component = associated_component.unwrap_or(nullptr);

        todo!();
        /*
        
        */
    }

    /**
      | Shows a dialog box using the specified
      | options.
      | 
      | The box is shown modally, and the method
      | will block until the user dismisses
      | it.
      | 
      | -----------
      | @param options
      | 
      | the options to use when creating the
      | dialog.
      | 
      | -----------
      | @return
      | 
      | the index of the button that was clicked.
      | 
      | @see MessageBoxOptions
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show(options: &MessageBoxOptions) -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Shows a dialog box using the specified
      | options.
      | 
      | The box will be displayed and placed
      | into a modal state, but this method will
      | return immediately, and the callback
      | will be invoked later when the user dismisses
      | the box.
      | 
      | -----------
      | @param options
      | 
      | the options to use when creating the
      | dialog.
      | ----------
      | @param callback
      | 
      | if this is non-null, the callback will
      | receive a call to its modalStateFinished()
      | when the box is dismissed with the index
      | of the button that was clicked as its
      | argument.
      | 
      | The callback object will be owned and
      | deleted by the system, so make sure that
      | it works safely and doesn't keep any
      | references to objects that might be
      | deleted before it gets called.
      | 
      | @see MessageBoxOptions
      |
      */
    pub fn show_async_with_modal_callback(
        options:  &MessageBoxOptions,
        callback: *mut dyn ModalComponentManagerCallback)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Shows a dialog box using the specified
      | options.
      | 
      | The box will be displayed and placed
      | into a modal state, but this method will
      | return immediately, and the callback
      | will be invoked later when the user dismisses
      | the box.
      | 
      | -----------
      | @param options
      | 
      | the options to use when creating the
      | dialog.
      | ----------
      | @param callback
      | 
      | if this is non-null, the callback will
      | be called when the box is dismissed with
      | the index of the button that was clicked
      | as its argument.
      | 
      | @see MessageBoxOptions
      |
      */
    pub fn show_async(
        options:  &MessageBoxOptions,
        callback: fn(_0: i32) -> ())  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Shows a dialog box that just has a message
      | and a single 'ok' button to close it.
      | 
      | The box will be displayed and placed
      | into a modal state, but this method will
      | return immediately, and the callback
      | will be invoked later when the user dismisses
      | the box.
      | 
      | -----------
      | @param iconType
      | 
      | the type of icon to show.
      | ----------
      | @param title
      | 
      | the headline to show at the top of the
      | box.
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the title.
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      | ----------
      | @param callback
      | 
      | if this is non-null, the callback will
      | receive a call to its modalStateFinished()
      | when the box is dismissed. The callback
      | object will be owned and deleted by the
      | system, so make sure that it works safely
      | and doesn't keep any references to objects
      | that might be deleted before it gets
      | called. You can use the ModalCallbackFunction
      | to easily pass in a lambda for this parameter.
      | 
      | @see ModalCallbackFunction
      |
      */
    pub fn show_message_box_async<'a>(
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback)  {

        todo!();
        /*
        
        */
    }

    /**
      | Shows a dialog box with two buttons.
      | 
      | Ideal for ok/cancel or yes/no choices.
      | The return key can also be used to trigger
      | the first button, and the escape key
      | for the second button.
      | 
      | If the callback parameter is null and
      | modal loops are enabled, the box is shown
      | modally, and the method will block until
      | the user has clicked the button (or pressed
      | the escape or return keys). If the callback
      | parameter is non-null, the box will
      | be displayed and placed into a modal
      | state, but this method will return immediately,
      | and the callback will be invoked later
      | when the user dismisses the box.
      | 
      | -----------
      | @param iconType
      | 
      | the type of icon to show.
      | ----------
      | @param title
      | 
      | the headline to show at the top of the
      | box.
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the title.
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      | ----------
      | @param callback
      | 
      | if this is non-null, the box will be launched
      | asynchronously, returning immediately,
      | and the callback will receive a call
      | to its modalStateFinished() when the
      | box is dismissed, with its parameter
      | being 1 if the ok button was pressed,
      | or 0 for cancel, The callback object
      | will be owned and deleted by the system,
      | so make sure that it works safely and
      | doesn't keep any references to objects
      | that might be deleted before it gets
      | called. You can use the ModalCallbackFunction
      | to easily pass in a lambda for this parameter.
      | 
      | -----------
      | @return
      | 
      | true if button 1 was clicked, false if
      | it was button 2. If the callback parameter
      | is not null, the method always returns
      | false, and the user's choice is delivered
      | later by the callback.
      | 
      | @see ModalCallbackFunction
      |
      */
    pub fn show_ok_cancel_box<'a>(
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback) -> bool {

        todo!();
        /*
        
        */
    }

    /**
      | Shows a dialog box with three buttons.
      | 
      | Ideal for yes/no/cancel boxes.
      | 
      | The escape key can be used to trigger
      | the third button.
      | 
      | If the callback parameter is null and
      | modal loops are enabled, the box is shown
      | modally, and the method will block until
      | the user has clicked the button (or pressed
      | the escape or return keys). If the callback
      | parameter is non-null, the box will
      | be displayed and placed into a modal
      | state, but this method will return immediately,
      | and the callback will be invoked later
      | when the user dismisses the box.
      | 
      | -----------
      | @param iconType
      | 
      | the type of icon to show.
      | ----------
      | @param title
      | 
      | the headline to show at the top of the
      | box.
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the title.
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      | ----------
      | @param callback
      | 
      | if this is non-null, the box will be launched
      | asynchronously, returning immediately,
      | and the callback will receive a call
      | to its modalStateFinished() when the
      | box is dismissed, with its parameter
      | being 1 if the "yes" button was pressed,
      | 2 for the "no" button, or 0 if it was cancelled,
      | The callback object will be owned and
      | deleted by the system, so make sure that
      | it works safely and doesn't keep any
      | references to objects that might be
      | deleted before it gets called. You can
      | use the
      | 
      | ModalCallbackFunction to easily pass
      | in a lambda for this parameter.
      | 
      | -----------
      | @return
      | 
      | If the callback parameter has been set,
      | this returns 0. Otherwise, it returns
      | one of the following values:
      | 
      | - 0 if 'cancel' was pressed
      | 
      | - 1 if 'yes' was pressed
      | 
      | - 2 if 'no' was pressed
      | 
      | @see ModalCallbackFunction
      |
      */
    pub fn show_yes_no_cancel_box<'a>(
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback) -> i32 {

        todo!();
        /*
        
        */
    }

    /**
      | Shows a dialog box with two buttons.
      | 
      | Ideal for yes/no boxes.
      | 
      | The escape key can be used to trigger
      | the no button.
      | 
      | If the callback parameter is null and
      | modal loops are enabled, the box is shown
      | modally, and the method will block until
      | the user has clicked the button (or pressed
      | the escape or return keys). If the callback
      | parameter is non-null, the box will
      | be displayed and placed into a modal
      | state, but this method will return immediately,
      | and the callback will be invoked later
      | when the user dismisses the box.
      | 
      | -----------
      | @param iconType
      | 
      | the type of icon to show.
      | ----------
      | @param title
      | 
      | the headline to show at the top of the
      | box.
      | ----------
      | @param message
      | 
      | a longer, more descriptive message
      | to show underneath the title.
      | ----------
      | @param associatedComponent
      | 
      | if this is non-null, it specifies the
      | component that the alert window should
      | be associated with. Depending on the
      | look and feel, this might be used for
      | positioning of the alert window.
      | ----------
      | @param callback
      | 
      | if this is non-null, the box will be launched
      | asynchronously, returning immediately,
      | and the callback will receive a call
      | to its modalStateFinished() when the
      | box is dismissed, with its parameter
      | being 1 if the "yes" button was pressed
      | or 0 for the "no" button was pressed.
      | The callback object will be owned and
      | deleted by the system, so make sure that
      | it works safely and doesn't keep any
      | references to objects that might be
      | deleted before it gets called. You can
      | use the
      | 
      | ModalCallbackFunction to easily pass
      | in a lambda for this parameter.
      | 
      | -----------
      | @return
      | 
      | If the callback parameter has been set,
      | this returns 0. Otherwise, it returns
      | one of the following values:
      | 
      | - 0 if 'no' was pressed
      | 
      | - 1 if 'yes' was pressed
      | 
      | @see ModalCallbackFunction
      |
      */
    pub fn show_yes_no_box<'a>(
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback) -> i32 {

        todo!();
        /*
        
        */
    }
    
    #[cfg(target_os="linux")]
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_message_box(&mut self, 
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component)  {
        
        todo!();
        /*
            AlertWindow::showMessageBox (iconType, title, message);
        */

    }
    
    #[cfg(target_os="linux")]
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show(&mut self, options: &MessageBoxOptions) -> i32 {
        
        todo!();
        /*
            return showDialog (options, nullptr, Async::no);
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn show_message_box_async(
        &mut self, 
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback

    ) {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (iconType, title, message, {}, associatedComponent, callback);
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn show_ok_cancel_box(&mut self, 
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback) -> bool {
        
        todo!();
        /*
            return AlertWindow::showOkCancelBox (iconType, title, message, {}, {}, associatedComponent, callback);
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn show_yes_no_cancel_box(&mut self, 
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback) -> i32 {
        
        todo!();
        /*
            return AlertWindow::showYesNoCancelBox (iconType, title, message, {}, {}, {},
                                                associatedComponent, callback);
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn show_yes_no_box(&mut self, 
        icon_type:            MessageBoxIconType,
        title:                &String,
        message:              &String,
        associated_component: *mut Component<'a>,
        callback:             *mut dyn ModalComponentManagerCallback) -> i32 {
        
        todo!();
        /*
            return AlertWindow::showOkCancelBox (iconType, title, message, TRANS("Yes"), TRANS("No"),
                                             associatedComponent, callback);
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn show_async_with_modal_callback(
        &mut self, 
        options:  &MessageBoxOptions,
        callback: *mut dyn ModalComponentManagerCallback

    )  {
        
        todo!();
        /*
            showDialog (options, callback, Async::yes);
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn show_async(&mut self, 
        options:  &MessageBoxOptions,
        callback: fn(_0: i32) -> ())  {
        
        todo!();
        /*
            showAsync (options, ModalCallbackFunction::create (callback));
        */

    }
}
